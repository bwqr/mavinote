use std::sync::Arc;

use base::{Error, Config};
use sqlx::{Pool, Sqlite, pool::PoolConnection};

use super::db;
use crate::{models::{AccountKind, State as ModelState, Account, Mavinote}, accounts::mavinote::MavinoteClient};

pub async fn sync() -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;
    let config = runtime::get::<Arc<Config>>().unwrap();

    let accounts = db::fetch_accounts(&mut conn).await?;

    for account in accounts {
        sync_account(&mut conn, &config, account).await?;
    }

    super::FOLDERS.get().unwrap().send_replace(db::fetch_folders(&mut conn).await.map_err(|e| e.into()).into());

    Ok(())
}

async fn sync_account(conn: &mut PoolConnection<Sqlite>, config: &Arc<Config>, account: Account) -> Result<(), Error> {
    if AccountKind::Mavinote != account.kind {
        log::debug!("only mavinote accounts are synced");

        return Ok(());
    }

    let account_data = db::fetch_account_data::<Mavinote>(conn, account.id).await?.unwrap();

    let mavinote = MavinoteClient::new(Some(account.id), config.api_url.clone(), account_data.token);

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
            db::create_folder(conn, Some(remote_folder.id()), account.id, remote_folder.name.clone()).await?
        };

        let commits = match mavinote.fetch_commits(remote_folder.id()).await {
            Ok(commits) => commits,
            Err(e) => {
                log::debug!("failed to fetch commits for folder with remote id {}, {e:?}", remote_folder.id);

                continue
            }
        };

        for commit in commits {
            let note = db::fetch_note_by_remote_id(conn, commit.note_id(), folder.local_id()).await?;

            if let Some(note) = note {
                if ModelState::Clean == note.state && note.commit_id < commit.commit_id {
                    // A note fetched by its remote id must have remote id. Hence we can safely unwrap it
                    match mavinote.fetch_note(note.remote_id().unwrap()).await {
                        Ok(Some(remote_note)) => db::update_note(
                            conn,
                            note.local_id(),
                            remote_note.title.as_ref().map(|title| title.as_str()),
                            remote_note.text.as_str(),
                            remote_note.commit_id,
                            ModelState::Clean,
                        ).await?,
                        Ok(None) => log::debug!("note with remote id {} does not exist on remote", note.remote_id().unwrap().0),
                        Err(e) => log::debug!("failed to fetch note with remote id {}, {e:?}", note.remote_id().unwrap().0),
                    }
                }
            } else {
                match mavinote.fetch_note(commit.note_id()).await {
                    Ok(Some(note)) => {
                        db::create_note(conn, folder.local_id(), Some(note.id()), note.title, note.text, note.commit_id).await?;
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
            match mavinote.create_folder(local_folder.name.as_str()).await {
                Ok(remote_folder) => {
                    sqlx::query("update folders set remote_id = ? where id = ?")
                        .bind(remote_folder.id)
                        .bind(local_folder.id)
                        .execute(&mut *conn)
                        .await?;

                    remote_folder_id = Some(remote_folder.id());
                },
                Err(e) => log::error!("failed to create local folder in remote, {e:?}"),
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
                        match mavinote.update_note(remote_id, local_note.title.as_ref().map(|title| title.as_str()), local_note.text.as_str()).await {
                            Ok(commit) => db::update_commit(conn, local_note.local_id(), commit.commit_id).await?,
                            Err(e) => log::debug!("failed to update note with id {}, {e:?}", local_note.id),
                        }
                    }
                } else if local_note.remote_id().is_none() {
                    match mavinote.create_note(remote_folder_id, local_note.title.as_ref().map(|title| title.as_str()), local_note.text.as_str()).await {
                        Ok(remote_note) => {
                            sqlx::query("update notes set remote_id = ?, commit_id = ? where id = ?")
                                .bind(remote_note.id)
                                .bind(remote_note.commit_id)
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
