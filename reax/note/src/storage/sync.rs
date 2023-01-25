use std::sync::Arc;

use sqlx::{Pool, Sqlite, pool::PoolConnection};

use super::db;
use crate::Error;
use crate::accounts::mavinote::{CreateFolderRequest, CreateNoteRequest};
use crate::models::{AccountKind, State as ModelState, Account};

pub async fn sync() -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let mavinote_accounts = db::fetch_accounts(&mut conn).await?
        .into_iter()
        .filter(|acc| acc.kind == AccountKind::Mavinote);

    for account in mavinote_accounts {
        sync_mavinote_account(&mut conn, account).await?;
    }

    super::FOLDERS.get().unwrap().send_replace(db::fetch_folders(&mut conn).await.map_err(|e| e.into()).into());

    Ok(())
}

async fn sync_mavinote_account(conn: &mut PoolConnection<Sqlite>, account: Account) -> Result<(), Error> {
    log::debug!("syncing mavinote account with id {}", account.id);

    let mavinote = super::mavinote_client(account.id).await.unwrap().unwrap();

    let remote_folders = mavinote.fetch_folders().await?;

    log::debug!("fetched folders length {}", remote_folders.len());

    for remote_folder in remote_folders {
        if let ModelState::Deleted = remote_folder.state {
            db::delete_folder_by_remote_id(conn, remote_folder.id(), account.id).await?;
            continue
        }

        let folder = if let Some(folder) = db::fetch_folder_by_remote_id(conn, remote_folder.id(), account.id).await? {
            folder
        } else {
            if let Some(device_folder) = &remote_folder.device_folder {
                db::create_folder(conn, Some(remote_folder.id()), account.id, device_folder.name.clone()).await?
            } else {
                log::warn!("A folder with no device folder is received. Some other devices must create our device folder");
                continue;
            }
        };

        let commits = match mavinote.fetch_commits(remote_folder.id()).await {
            Ok(commits) => commits,
            Err(e) => {
                log::error!("failed to fetch commits for folder with remote id {}, {e:?}", remote_folder.id);

                continue
            }
        };

        for commit in commits {
            let note = db::fetch_note_by_remote_id(conn, commit.note_id(), folder.local_id()).await?;

            if let Some(note) = note {
                if ModelState::Clean == note.state && note.commit < commit.commit {
                    // A note fetched by its remote id must have remote id. Hence we can safely unwrap it
                    match mavinote.fetch_note(note.remote_id().unwrap()).await {
                        Ok(Some(remote_note)) => {
                            if let Some(device_note) = remote_note.device_note {
                                db::update_note(
                                    conn,
                                    note.local_id(),
                                    device_note.title.as_ref().map(|title| title.as_str()),
                                    device_note.text.as_str(),
                                    remote_note.commit,
                                    ModelState::Clean,
                                ).await?
                            } else {
                                log::warn!("A note with no device note is received. Some other devices must create our device note");
                            }
                        },
                        Ok(None) => log::warn!("note with remote id {} does not exist on remote", note.remote_id().unwrap().0),
                        Err(e) => log::error!("failed to fetch note with remote id {}, {e:?}", note.remote_id().unwrap().0),
                    }
                }
            } else {
                match mavinote.fetch_note(commit.note_id()).await {
                    Ok(Some(remote_note)) => {
                        let (remote_id, remote_commit) = (remote_note.id(), remote_note.commit);
                        if let Some(device_note) = remote_note.device_note {
                            db::create_note(
                                conn,
                                folder.local_id(),
                                Some(remote_id),
                                device_note.title,
                                device_note.text,
                                remote_commit
                            ).await?;
                        } else {
                            log::warn!("A note with no device note is received. Some other devices must create our device note");
                        }
                    },
                    Ok(None) => log::debug!("note with remote id {} does not exist on remote", commit.note_id().0),
                    Err(e) => log::debug!("failed to fetch note with remote id {}, {e:?}", commit.note_id),
                };
            }
        }
    }

    let local_folders = db::fetch_account_folders(conn, account.id).await?;

    for local_folder in local_folders {
        if let ModelState::Deleted = local_folder.state {
            if let Some(remote_id) = local_folder.remote_id() {
                if let Ok(_) = mavinote.delete_folder(remote_id).await {
                    db::delete_folder(conn, local_folder.local_id()).await?;
                }
            } else {
                log::error!("A folder cannot be in deleted state while being not created at remote side");
            }

            continue;
        }

        let mut remote_folder_id = local_folder.remote_id();

        if remote_folder_id.is_none() {
            for _ in 0..2 {
                let devices = db::devices(conn, account.id).await?;

                let request = devices
                    .into_iter()
                    .map(|device| CreateFolderRequest{ device_id: device.id, name: &local_folder.name })
                    .collect();

                match mavinote.create_folder(request).await {
                    Ok(remote_folder) => {
                        db::update_folder_remote_id(conn, local_folder.local_id(), remote_folder.id()).await?;

                        remote_folder_id = Some(remote_folder.id());

                        break;
                    },
                    Err(crate::accounts::mavinote::Error::Message(msg)) if msg == "devices_mismatch" => {
                        let new_devices = mavinote.fetch_devices().await?
                            .into_iter()
                            .map(|dev| dev.id)
                            .collect::<Vec<i32>>();

                        db::delete_devices(conn, account.id).await?;

                        if !new_devices.is_empty() {
                            db::create_devices(conn, account.id, &new_devices).await?;
                        }
                    },
                    Err(e) => {
                        log::error!("failed to create folder in remote {e:?}");
                        break;
                    },
                }
            }
        };

        if let Some(remote_folder_id) = remote_folder_id {
            let local_notes = db::fetch_all_notes(conn, local_folder.local_id()).await?;

            for local_note in local_notes {
                if let ModelState::Deleted = local_note.state {
                    if let Some(remote_id) = local_note.remote_id() {
                        if let Ok(_) = mavinote.delete_note(remote_id).await {
                            db::delete_note(conn, local_note.local_id()).await?;
                        }
                    } else {
                        log::error!("A note cannot be in deleted state while being not created at remote side");
                    }
                } else if let Some(remote_id) = local_note.remote_id() {
                    if ModelState::Modified == local_note.state {

                        let devices = db::devices(conn, account.id).await?;
                        let device_notes = devices
                            .into_iter()
                            .map(|device| CreateNoteRequest{ device_id: device.id, title: local_note.title.as_ref().map(|title| title.as_str()), text: &local_note.text })
                            .collect();

                        match mavinote.update_note(remote_id, local_note.commit, device_notes).await {
                            Ok(commit) => db::update_commit(conn, local_note.local_id(), commit.commit).await?,
                            Err(e) => log::debug!("failed to update note with id {}, {e:?}", local_note.id),
                        }
                    }
                } else if local_note.remote_id().is_none() {
                    let devices = db::devices(conn, account.id).await?;
                    let device_notes = devices
                        .into_iter()
                        .map(|device| CreateNoteRequest{ device_id: device.id, title: local_note.title.as_ref().map(|title| title.as_str()), text: &local_note.text })
                        .collect();

                    match mavinote.create_note(remote_folder_id, device_notes).await {
                        Ok(remote_note) => {
                            sqlx::query("update notes set remote_id = ?, 'commit' = ? where id = ?")
                                .bind(remote_note.id)
                                .bind(remote_note.commit)
                                .bind(local_note.id)
                                .execute(&mut *conn)
                                .await?;
                        },
                        Err(e) => log::debug!("failed to create local note in remote, {e:?}"),
                    }
                }
            }
        }
    }

    Ok(())
}
