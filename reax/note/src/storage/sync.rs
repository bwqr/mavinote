use std::collections::{HashSet, HashMap};
use std::sync::Arc;
use std::time::Duration;

use base::Config;
use futures_util::{StreamExt, FutureExt, SinkExt};
use sqlx::{Pool, Sqlite, pool::PoolConnection};
use tokio::sync::watch::{channel, Receiver};
use tokio::time::Instant;
use tokio_tungstenite::connect_async;
use x25519_dalek::StaticSecret;

use super::db;
use crate::crypto::{DeviceCipher, Error as CryptoError};
use crate::{Error, crypto};
use crate::accounts::mavinote::{CreateFolderRequest, CreateNoteRequest, MavinoteClient, Error as MavinoteError, RespondFolderRequest, RespondRequests, RespondNoteRequest, CreateRequests, DeviceMessage};
use crate::models::{AccountKind, State as ModelState, RemoteId, Note, Mavinote, LocalId};

const PING_INTERVAL: u64 = 30;

struct Sync<'a> {
    account_id: i32,
    client: MavinoteClient,
    privkey: &'a StaticSecret,
    ciphers: Vec<DeviceCipher>,
}

impl<'a> Sync<'a> {
    pub async fn sync(mut self, conn: &mut PoolConnection<Sqlite>) -> Result<(), Error> {
        self.ciphers = self.devices(conn).await?;

        self.remote(conn).await?;

        self.local(conn).await?;

        self.respond_device_requests(conn).await?;

        Ok(())
    }

    async fn devices(&self, conn: &mut PoolConnection<Sqlite>) -> Result<Vec<DeviceCipher>, Error> {
        let new_devices = self.client.fetch_devices().await?;

        // Verify and update the pubkeys
        let ciphers = new_devices
            .iter()
            .map(|device| DeviceCipher::try_from_key(device.id, self.privkey, &device.pubkey))
            .collect::<Result<Vec<_>, CryptoError>>()?;

        let old_devices = db::fetch_devices(conn, self.account_id).await?;

        let devices_to_delete = old_devices.iter()
            .filter(|od| new_devices.iter().find(|nd| od.id == nd.id && od.pubkey == nd.pubkey).is_none())
            .map(|od| od.id)
            .collect::<Vec<_>>();

        let devices_to_create = new_devices.into_iter()
            .filter(|nd| old_devices.iter().find(|od| od.id == nd.id && od.pubkey == nd.pubkey).is_none())
            .collect::<Vec<_>>();

        if !devices_to_delete.is_empty() {
            db::delete_devices(conn, self.account_id, &devices_to_delete).await?;
        }

        if !devices_to_create.is_empty() {
            db::create_devices(conn, self.account_id, &devices_to_create).await?;
        }

        Ok(ciphers)
    }

    async fn remote(&self, conn: &mut PoolConnection<Sqlite>) -> Result<(), Error> {
        let mut requests = CreateRequests::default();

        let remote_folders = self.client.fetch_folders().await?;
        for remote_folder in remote_folders {
            let reqs = self.remote_folder(conn, remote_folder).await?;
            requests.folder_ids.extend(reqs.folder_ids);
            requests.note_ids.extend(reqs.note_ids);
        }

        if !requests.folder_ids.is_empty() || !requests.note_ids.is_empty() {
            self.client.create_requests(&requests).await?;
        }

        Ok(())
    }

    async fn remote_folder(&self, conn: &mut PoolConnection<Sqlite>, remote_folder: crate::accounts::mavinote::responses::Folder) -> Result<CreateRequests, Error> {
        if let ModelState::Deleted = remote_folder.state {
            return db::delete_folder_by_remote_id(conn, remote_folder.id(), self.account_id)
                .await
                .map(|_| CreateRequests::default())
                .map_err(|e| e.into());
        }

        let mut requests = CreateRequests::default();
        let commits = self.client.fetch_commits(remote_folder.id()).await?;

        let folder = match db::fetch_folder_by_remote_id(conn, remote_folder.id(), self.account_id).await? {
            Some(folder) => folder,
            None => {
                let Some(device_folder) = &remote_folder.device_folder else {
                    log::debug!("A folder with no device folder is received. Some other devices must create our device folder");

                    requests.folder_ids.push(remote_folder.id);
                    requests.note_ids.extend(commits.into_iter().map(|commit| commit.note_id));

                    return Ok(requests);
                };

                let Some(cipher) = self.ciphers.iter().find(|cipher| cipher.device_id == device_folder.sender_device_id) else {
                    log::warn!("A folder with unknown sender is received");
                    return Ok(requests);
                };

                db::create_folder(
                    conn,
                    Some(remote_folder.id()),
                    self.account_id,
                    cipher.decrypt(&device_folder.name)?
                ).await?
            }
        };

        for commit in commits {
            if self.remote_note(conn, commit.commit, commit.note_id(), folder.local_id()).await? {
                requests.note_ids.push(commit.note_id);
            }
        }

        Ok(requests)
    }

    async fn remote_note(&self, conn: &mut PoolConnection<Sqlite>, commit: i32, note_id: RemoteId, folder_id: LocalId) -> Result<bool, Error> {
        let local_note = db::fetch_note_by_remote_id(conn, note_id, folder_id).await?;

        if let Some(note) = &local_note {
            // Having same commit means there is no need to pull fresh note from the server.
            // Deleted state will be handled by local sync
            if note.state == ModelState::Deleted || note.commit >= commit {
                return Ok(false);
            }
        }
        let Some(remote_note) = self.client.fetch_note(note_id).await? else {
            log::warn!("note with remote id {} does not exist on remote", note_id.0);
            return Ok(false);
        };

        let Some(device_note) = remote_note.device_note else {
            log::debug!("A note with no device note is received. Some other devices must create our device note");
            return Ok(true);
        };

        let Some(cipher) = self.ciphers.iter().find(|cipher| cipher.device_id == device_note.sender_device_id) else {
            log::warn!("A note with unknown sender device is received");
            return Ok(false);
        };

        let name = cipher.decrypt(&device_note.name)?;
        let mut text = cipher.decrypt(&device_note.text)?;

        if let Some(note) = local_note {
            // Merge local and remote text to resolve the conflict
            if note.state == ModelState::Modified {
                text += "\n___CONFLICT_RESOLVING___\n";
                text += &note.text;
            }

            db::update_note(
                conn,
                note.local_id(),
                &name,
                &text,
                remote_note.commit,
                note.state,
            ).await?
        } else {
            db::create_note(
                conn,
                folder_id,
                Some(note_id),
                name,
                text,
                remote_note.commit
            ).await?;
        }

        Ok(false)
    }

    async fn local(&self, conn: &mut PoolConnection<Sqlite>) -> Result<(), Error> {
        let local_folders = db::fetch_account_folders(conn, self.account_id).await?;
        for local_folder in local_folders {
            self.local_folder(conn, local_folder).await?;
        }

        Ok(())
    }

    async fn local_folder(&self, conn: &mut PoolConnection<Sqlite>, local_folder: crate::models::Folder) -> Result<(), Error> {
        if let ModelState::Deleted = local_folder.state {
            let Some(remote_id) = local_folder.remote_id() else {
                return Err(Error::Unreachable("Deleted folder without a remote id cannot exist"));
            };

            self.client.delete_folder(remote_id).await?;

            return db::delete_folder(conn, local_folder.local_id()).await.map_err(|e| e.into());
        }

        let remote_folder_id = match local_folder.remote_id() {
            Some(id) => id,
            None => {
                let mut request = vec![];
                let nonces = db::unique_nonces(conn, &self.ciphers.iter().map(|c| c.device_id).collect::<Vec<_>>()).await?;
                for (cipher, nonce) in self.ciphers.iter().zip(nonces) {
                    request.push(CreateFolderRequest{ device_id: cipher.device_id, name: cipher.encrypt(&local_folder.name, nonce)? });
                }

                let remote_folder = self.client.create_folder(&request).await?;

                db::update_folder_remote_id(conn, local_folder.local_id(), remote_folder.id()).await?;

                remote_folder.id()
            }
        };

        let local_notes = db::fetch_all_notes(conn, local_folder.local_id()).await?;

        for local_note in local_notes {
            if let ModelState::Deleted = local_note.state {
                let Some(remote_id) = local_note.remote_id() else {
                    return Err(Error::Unreachable("Deleted note without a remote id cannot exist"));
                };

                self.client.delete_note(remote_id).await?;

                db::delete_note(conn, local_note.local_id()).await?;

                continue;
            }

            // This state does not need sync
            if local_note.state != ModelState::Modified && local_note.remote_id().is_some() {
                continue;
            }
            let mut device_notes = vec![];
            let name_nonces = db::unique_nonces(conn, &self.ciphers.iter().map(|c| c.device_id).collect::<Vec<_>>()).await?;
            let text_nonces = db::unique_nonces(conn, &self.ciphers.iter().map(|c| c.device_id).collect::<Vec<_>>()).await?;

            for (cipher, (name_nonce, text_nonce)) in self.ciphers.iter().zip(name_nonces.into_iter().zip(text_nonces)) {
                device_notes.push(CreateNoteRequest{
                    device_id: cipher.device_id,
                    name: cipher.encrypt(&local_note.name, name_nonce)?,
                    text: cipher.encrypt(&local_note.text, text_nonce)?
                });
            }

            if let Some(remote_id) = local_note.remote_id() {
                let commit = self.client.update_note(remote_id, local_note.commit, &device_notes).await?;
                db::update_commit(conn, local_note.local_id(), commit.commit).await?;
            } else {
                let remote_note = self.client.create_note(remote_folder_id, &device_notes).await?;
                sqlx::query("update notes set remote_id = ?, 'commit' = ? where id = ?")
                    .bind(remote_note.id)
                    .bind(remote_note.commit)
                    .bind(local_note.id)
                    .execute(&mut *conn)
                    .await?;
            }
        }

        Ok(())
    }

    async fn respond_device_requests(&self, conn: &mut PoolConnection<Sqlite>) -> Result<(), Error> {
        let requests = self.client.fetch_requests().await?;

        let mut note_ids: HashMap<i32, HashSet<&DeviceCipher>> = HashMap::new();
        let mut folder_ids: HashMap<i32, HashSet<&DeviceCipher>> = HashMap::new();

        for req in &requests.folder_requests {
            if let Some(cipher) = self.ciphers.iter().find(|c| c.device_id == req.device_id) {
                folder_ids.entry(req.folder_id).or_insert(HashSet::new()).insert(cipher);
            }
        }

        for req in &requests.note_requests {
            if let Some(cipher) = self.ciphers.iter().find(|c| c.device_id == req.device_id) {
                note_ids.entry(req.note_id).or_insert(HashSet::new()).insert(cipher);
            }
        }

        let mut device_responses: HashMap<i32, RespondRequests> = HashMap::new();

        for (folder_id, ciphers) in folder_ids {
            let folder = db::fetch_folder_by_remote_id(conn, RemoteId(folder_id), self.account_id).await?;

            if let Some(folder) = folder {
                let nonces = db::unique_nonces(conn, &self.ciphers.iter().map(|c| c.device_id).collect::<Vec<_>>()).await?;

                for (cipher, nonce) in ciphers.into_iter().zip(nonces) {
                    device_responses
                        .entry(cipher.device_id)
                        .or_insert(RespondRequests { device_id: cipher.device_id, folders: Vec::new(), notes: Vec::new() })
                        .folders
                        .push(RespondFolderRequest { folder_id, name: cipher.encrypt(&folder.name, nonce)? });
                }
            }
        }

        for (note_id, ciphers) in note_ids {
            let note: Option<Note> = sqlx::query_as("select notes.* from notes inner join folders on folders.id = notes.folder_id where notes.remote_id = ? and folders.account_id = ?")
                .bind(note_id)
                .bind(self.account_id)
                .fetch_optional(&mut *conn)
                .await?;

            if let Some(note) = note {
                let name_nonces = db::unique_nonces(conn, &ciphers.iter().map(|c| c.device_id).collect::<Vec<_>>()).await?;
                let text_nonces = db::unique_nonces(conn, &ciphers.iter().map(|c| c.device_id).collect::<Vec<_>>()).await?;

                for (cipher, (name_nonce, text_nonce)) in ciphers.iter().zip(name_nonces.into_iter().zip(text_nonces)) {
                    device_responses
                        .entry(cipher.device_id)
                        .or_insert(RespondRequests { device_id: cipher.device_id, folders: Vec::new(), notes: Vec::new() })
                        .notes
                        .push(RespondNoteRequest { note_id, name: cipher.encrypt(&note.name, name_nonce)?, text: cipher.encrypt(&note.text, text_nonce)? });
                }
            }
        }

        for resp in device_responses.values() {
            self.client.respond_requests(resp).await?;
        }

        Ok(())
    }

    async fn load_device_ciphers(conn: &mut PoolConnection<Sqlite>, privkey: &StaticSecret, account_id: i32) -> Result<Vec<DeviceCipher>, Error> {
        let devices = db::fetch_devices(conn, account_id).await?;

        devices
            .into_iter()
            .map(|device| DeviceCipher::try_from_key(device.id, privkey, &device.pubkey))
            .collect::<Result<Vec<_>, CryptoError>>()
            .map_err(|e| e.into())
    }
}

pub async fn sync() -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;
    let privkey = crypto::load_privkey(&mut conn).await?;

    let mavinote_accounts = db::fetch_accounts(&mut conn).await?
        .into_iter()
        .filter(|acc| acc.kind == AccountKind::Mavinote);

    for account in mavinote_accounts {
        log::debug!("syncing mavinote account with id {}", account.id);

        let Some(client) = super::mavinote_client(&mut conn, account.id).await? else {
            return Err(Error::Unreachable("Mavinote account must have a client"));
        };

        let sync = Sync {
            account_id: account.id,
            client,
            privkey: &privkey,
            ciphers: Sync::load_device_ciphers(&mut conn, &privkey, account.id).await?
        };

        if let Err(e) = sync.sync(&mut conn).await {
            match e {
                Error::Mavinote(MavinoteError::DeviceDeleted(_)) => return Err(e),
                Error::Mavinote(MavinoteError::Unauthorized(_)) => {
                    log::error!("Sync is failed due to unauthorized error");

                    if let Err(e) = super::login(account.id).await {
                        log::debug!("Unable to login after unauthorized error while syncing, {e:?}");
                    }
                }
                e => log::error!("Failed to sync mavinote account with id {}, {e:?}", account.id),
            }
        }
    }

    super::update_send_folders(&mut conn).await;

    Ok(())
}

pub async fn listen_notifications(account_id: i32) -> Result<Receiver<()>, Error> {
    let (tx, rx) = channel(());

    let token = {
        let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;
        match db::fetch_account_data::<Mavinote>(&mut conn, account_id).await {
            Ok(Some(mavinote)) => mavinote.token,
            Ok(None) => return Err(super::NOT_MAVINOTE_ACCOUNT),
            Err(sqlx::Error::ColumnDecode { .. }) => return Err(super::NOT_MAVINOTE_ACCOUNT),
            Err(e) => return Err(e.into()),
        }
    };

    let ws_url = runtime::get::<Arc<Config>>().unwrap().ws_url.clone();

    tokio::spawn(async move {
        let mut wait = 2;

        loop {
            let Ok((mut sock, _)) = connect_async(format!("{}/user/notifications?token={}", ws_url, token)).await else {
                if tx.is_closed() {
                    return;
                }

                tokio::time::sleep(std::time::Duration::from_secs(wait)).await;

                wait = std::cmp::min(32, wait * 2);

                continue;
            };

            wait = 2;

            let mut instant = Instant::now();
            loop {
                if Instant::now().duration_since(instant).as_secs() > PING_INTERVAL {
                    instant = Instant::now();
                    if let Err(e) = sock.send("ping".into()).await {
                        log::error!("failed to ping over socket, {e:?}");
                    }
                }

                let stream = tokio::time::timeout_at(instant + Duration::from_secs(PING_INTERVAL), sock.next()).fuse();
                let close_check = tx.closed().fuse();

                futures_util::pin_mut!(stream);
                futures_util::pin_mut!(close_check);

                let Ok(res) = futures_util::select! {
                    res = stream => res,
                    _ = close_check => return,
                } else {
                    // continue to next loop to send a ping message
                    continue
                };

                let Some(frame) = res else {
                    log::debug!("Socket stream is ended");
                    break;
                };

                let msg = match frame {
                    Ok(msg) => msg,
                    Err(e) => {
                        log::debug!("Error on socket {e:?}");
                        break;
                    },
                };

                let text = match msg.into_text() {
                    Ok(text) => text,
                    Err(e) => {
                        log::debug!("non text message is received, {e:?}");
                        continue;
                    }
                };

                let msg = match serde_json::from_str::<DeviceMessage>(&text) {
                    Ok(msg) => msg,
                    Err(e) => {
                        log::debug!("unexpected device message is received, {e:?}");
                        continue;
                    },
                };

                log::debug!("message is received {msg:?}");

                match handle_message(&msg, account_id).await {
                    Err(Error::Mavinote(crate::accounts::mavinote::Error::Unauthorized(_))) => {
                        if let Err(e) = super::login(account_id).await {
                            log::debug!("Unable to login after unauthorized error, {e:?}");
                        }

                        if let Err(e) = handle_message(&msg, account_id).await {
                            log::debug!("Failed to retry handling message, {e:?}");
                        }
                    }
                    Err(e) => log::debug!("failed to handle message, {e:?}"),
                    _ => { },
                }
            }
        }
    });

    Ok(rx)
}

pub(crate) async fn sync_devices(conn: &mut PoolConnection<Sqlite>, account_id: i32) -> Result<(), Error> {
    let privkey = crypto::load_privkey(conn).await?;

    let Some(client) = super::mavinote_client(conn, account_id).await? else {
        return Err(Error::Unreachable("Mavinote account must have a client"));
    };

    let sync = Sync {
        account_id,
        client,
        privkey: &privkey,
        ciphers: Vec::new()
    };

    sync.devices(conn).await
        .map(|_| ())
}

async fn handle_message(msg: &DeviceMessage, account_id: i32) -> Result<bool, Error> {
    match msg {
        DeviceMessage::RefreshRequests => refresh_respond_requests(account_id).await?,
        DeviceMessage::RefreshRemote => refresh_remote(account_id).await?,
        DeviceMessage::RefreshFolder(folder_id) => refresh_folder(account_id, *folder_id).await?,
        DeviceMessage::RefreshNote { folder_id, note_id } => refresh_note(account_id, *folder_id, *note_id).await?,
        DeviceMessage::Timeout => return Ok(true),
        _ => log::debug!("message is unhandled"),
    };

    Ok(false)
}

async fn refresh_respond_requests(account_id: i32) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let privkey = crypto::load_privkey(&mut conn).await?;

    let Some(client) = super::mavinote_client(&mut conn, account_id).await? else {
        return Err(Error::Unreachable("Mavinote account must have a client"));
    };

    let sync = Sync {
        account_id,
        client,
        privkey: &privkey,
        ciphers: Sync::load_device_ciphers(&mut conn, &privkey, account_id).await.unwrap()
    };

    sync.respond_device_requests(&mut conn).await
}

async fn refresh_remote(account_id: i32) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let privkey = crypto::load_privkey(&mut conn).await?;

    let Some(client) = super::mavinote_client(&mut conn, account_id).await? else {
        return Err(Error::Unreachable("Mavinote account must have a client"));
    };

    let sync = Sync {
        account_id,
        client,
        privkey: &privkey,
        ciphers: Sync::load_device_ciphers(&mut conn, &privkey, account_id).await.unwrap()
    };

    sync.remote(&mut conn).await?;

    super::update_send_folders(&mut conn).await;

    Ok(())
}

async fn refresh_folder(account_id: i32, folder_id: i32) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let Some(client) = super::mavinote_client(&mut conn, account_id).await? else {
        return Err(Error::Unreachable("Mavinote account must have a client"));
    };

    let Some(remote_folder) = client.fetch_folder(RemoteId(folder_id)).await? else {
        log::error!("Folder not found in remote while refreshing the folder");

        return Ok(());
    };

    let privkey = crypto::load_privkey(&mut conn).await?;

    let sync = Sync {
        account_id,
        client,
        privkey: &privkey,
        ciphers: Sync::load_device_ciphers(&mut conn, &privkey, account_id).await.unwrap()
    };

    sync.remote_folder(&mut conn, remote_folder).await?;

    super::update_send_folders(&mut conn).await;

    Ok(())
}

async fn refresh_note(account_id: i32, folder_id: i32, note_id: i32) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let Some(client) = super::mavinote_client(&mut conn, account_id).await? else {
        return Err(Error::Unreachable("Mavinote account must have a client"));
    };

    let Some(remote_note) = client.fetch_note(RemoteId(note_id)).await? else {
        log::error!("Note not found in remote while refreshing the note");

        return Ok(());
    };

    let Some(folder) = db::fetch_folder_by_remote_id(&mut conn, RemoteId(folder_id), account_id).await? else {
        log::error!("Folder belonging to note is not found");

        return Ok(());
    };

    let privkey = crypto::load_privkey(&mut conn).await?;

    let sync = Sync {
        account_id,
        client,
        privkey: &privkey,
        ciphers: Sync::load_device_ciphers(&mut conn, &privkey, account_id).await.unwrap()
    };

    sync.remote_note(&mut conn, remote_note.commit, remote_note.id(), folder.local_id()).await?;

    super::update_send_notes(&mut conn, folder.local_id()).await;

    Ok(())
}
