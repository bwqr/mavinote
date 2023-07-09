use std::collections::{HashSet, HashMap};
use std::sync::Arc;

use sqlx::{Pool, Sqlite, pool::PoolConnection};
use x25519_dalek::StaticSecret;

use super::db;
use crate::crypto::{DeviceCipher, Error as CryptoError};
use crate::{Error, crypto};
use crate::accounts::mavinote::{CreateFolderRequest, CreateNoteRequest, MavinoteClient, RespondFolderRequest, RespondRequests, RespondNoteRequest};
use crate::models::{AccountKind, State as ModelState, RemoteId, Note};

struct Sync<'a> {
    account_id: i32,
    client: MavinoteClient,
    privkey: &'a StaticSecret,
    ciphers: Vec<DeviceCipher>,
}

impl<'a> Sync<'a> {
    pub async fn sync(self, conn: &mut PoolConnection<Sqlite>) -> Result<(), Error> {
        self.devices(conn).await?;

        self.remote(conn).await?;

        self.local(conn).await?;

        self.respond_device_requests(conn).await?;

        Ok(())
    }

    async fn devices(&self, conn: &mut PoolConnection<Sqlite>) -> Result<(), Error> {
        let new_devices = self.client.fetch_devices().await?;

        // Verify that the pubkeys are valid
        for new_device in &new_devices {
            DeviceCipher::try_from_key(new_device.id, self.privkey, &new_device.pubkey)?;
        }

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

        Ok(())
    }

    async fn remote(&self, conn: &mut PoolConnection<Sqlite>) -> Result<(), Error> {
        let remote_folders = self.client.fetch_folders().await?;
        for remote_folder in remote_folders {
            self.remote_folder(conn, remote_folder).await?;
        }

        Ok(())
    }

    async fn remote_folder(&self, conn: &mut PoolConnection<Sqlite>, remote_folder: crate::accounts::mavinote::responses::Folder) -> Result<(), Error> {
        if let ModelState::Deleted = remote_folder.state {
            return db::delete_folder_by_remote_id(conn, remote_folder.id(), self.account_id).await.map_err(|e| e.into());
        }

        let folder = match db::fetch_folder_by_remote_id(conn, remote_folder.id(), self.account_id).await? {
            Some(folder) => folder,
            None => {
                let Some(device_folder) = &remote_folder.device_folder else {
                    log::warn!("A folder with no device folder is received. Some other devices must create our device folder");
                    return Ok(());
                };

                let Some(cipher) = self.ciphers.iter().find(|cipher| cipher.device_id == device_folder.sender_device_id) else {
                    log::warn!("A folder with unknown sender is received");
                    return Ok(());
                };

                db::create_folder(
                    conn,
                    Some(remote_folder.id()),
                    self.account_id,
                    cipher.decrypt(&device_folder.name)?
                ).await?
            }
        };

        let commits = self.client.fetch_commits(remote_folder.id()).await?;

        for commit in commits {
            match db::fetch_note_by_remote_id(conn, commit.note_id(), folder.local_id()).await? {
                Some(note) if note.state != ModelState::Clean || note.commit >= commit.commit => { } // This case will be handled by local syncing
                opt => {
                    let Some(remote_note) = self.client.fetch_note(commit.note_id()).await? else {
                        log::warn!("note with remote id {} does not exist on remote", commit.note_id().0);
                        continue;
                    };

                    let Some(device_note) = remote_note.device_note else {
                        log::warn!("A note with no device note is received. Some other devices must create our device note");
                        continue;
                    };

                    let Some(cipher) = self.ciphers.iter().find(|cipher| cipher.device_id == device_note.sender_device_id) else {
                        log::warn!("A note with no device note is received. Some other devices must create our device note");
                        continue;
                    };

                    let name = cipher.decrypt(&device_note.name)?;
                    let text = cipher.decrypt(&device_note.text)?;

                    if let Some(note) = opt {
                        db::update_note(
                            conn,
                            note.local_id(),
                            &name,
                            &text,
                            remote_note.commit,
                            ModelState::Clean,
                        ).await?
                    } else {
                        db::create_note(
                            conn,
                            folder.local_id(),
                            Some(commit.note_id()),
                            name,
                            text,
                            remote_note.commit
                        ).await?;
                    }
                },
            };
        }

        Ok(())
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
                for cipher in &self.ciphers {
                    request.push(CreateFolderRequest{ device_id: cipher.device_id, name: cipher.encrypt(&local_folder.name)? });
                }

                let remote_folder = self.client.create_folder(request).await?;

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
                return db::delete_note(conn, local_note.local_id()).await.map_err(|e| e.into());
            }

            match local_note.remote_id() {
                Some(remote_id) if local_note.state == ModelState::Modified => {
                    let mut device_notes = vec![];
                    for cipher in &self.ciphers {
                        device_notes.push(CreateNoteRequest{
                            device_id: cipher.device_id,
                            name: cipher.encrypt(&local_note.name)?,
                            text: cipher.encrypt(&local_note.text)?
                        });
                    }

                    let commit = self.client.update_note(remote_id, local_note.commit, device_notes).await?;

                    db::update_commit(conn, local_note.local_id(), commit.commit).await?
                }
                None => {
                    let mut device_notes = vec![];
                    for cipher in &self.ciphers {
                        device_notes.push(CreateNoteRequest{
                            device_id: cipher.device_id,
                            name: cipher.encrypt(&local_note.name)?,
                            text: cipher.encrypt(&local_note.text)?
                        });
                    }

                    let remote_note = self.client.create_note(remote_folder_id, device_notes).await?;
                    sqlx::query("update notes set remote_id = ?, 'commit' = ? where id = ?")
                        .bind(remote_note.id)
                        .bind(remote_note.commit)
                        .bind(local_note.id)
                        .execute(&mut *conn)
                        .await?;
                }
                _ => { }, // This case does not need syncing
            };
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
                for cipher in ciphers {
                    device_responses
                        .entry(cipher.device_id)
                        .or_insert(RespondRequests { device_id: cipher.device_id, folders: Vec::new(), notes: Vec::new() })
                        .folders
                        .push(RespondFolderRequest { folder_id, name: cipher.encrypt(&folder.name)? });
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
                for cipher in ciphers {
                    device_responses
                        .entry(cipher.device_id)
                        .or_insert(RespondRequests { device_id: cipher.device_id, folders: Vec::new(), notes: Vec::new() })
                        .notes
                        .push(RespondNoteRequest { note_id, name: cipher.encrypt(&note.name)?, text: cipher.encrypt(&note.text)? });
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
            log::error!("Failed to sync mavinote account with id {}, {e:?}", account.id);
        }
    }

    super::update_send_folders(&mut conn).await;

    Ok(())
}
