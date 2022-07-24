use std::sync::Arc;

use once_cell::sync::OnceCell;
use sqlx::{Pool, Sqlite};
use tokio::sync::watch::{channel, Sender};

use base::{Error, State, observable_map::{ObservableMap, Receiver}};

use models::{Folder, Note, State as ModelState, LocalId, Account, AccountKind};

pub mod models;
mod requests;
mod responses;
mod storage;
mod sync;

static ACCOUNTS: OnceCell<Sender<State<Vec<Account>, Error>>> = OnceCell::new();
static FOLDERS: OnceCell<Sender<State<Vec<Folder>, Error>>> = OnceCell::new();
static NOTES_MAP: OnceCell<Arc<ObservableMap<State<Vec<Note>, Error>>>> = OnceCell::new();
static ACTIVE_SYNCS: OnceCell<Sender<i32>> = OnceCell::new();

fn start_sync() {
    ACTIVE_SYNCS.get().unwrap().send_modify(|active_syncs| { *active_syncs += 1; });
}
fn end_sync() {
    ACTIVE_SYNCS.get().unwrap().send_modify(|active_syncs| { *active_syncs -= 1; });
}

pub fn init() {
    ACCOUNTS.set(channel(State::default()).0).unwrap();
    FOLDERS.set(channel(State::default()).0).unwrap();
    NOTES_MAP.set(Arc::new(ObservableMap::new())).unwrap();
    ACTIVE_SYNCS.set(channel(0).0).unwrap();
}

pub fn active_syncs() -> tokio::sync::watch::Receiver<i32> {
    ACTIVE_SYNCS.get().unwrap().subscribe()
}

pub async fn try_sync() -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let mavinote_account = if let Some(account) = storage::fetch_accounts(&mut conn).await?.into_iter().find(|a| a.kind == AccountKind::Mavinote) {
        account
    } else {
        log::debug!("mavinote account is not found, not syncing");
        return Ok(());
    };

    let remote_folders = requests::fetch_folders().await?;

    for remote_folder in remote_folders {
        if let ModelState::Deleted = remote_folder.state {
            storage::delete_folder_by_remote_id(&mut conn, remote_folder.id(), mavinote_account.id).await?;
            continue
        }

        let folder = if let Some(folder) = storage::fetch_folder_by_remote_id(&mut conn, remote_folder.id(), mavinote_account.id).await? {
            folder
        } else {
            storage::create_folder(&mut conn, Some(remote_folder.id()), mavinote_account.id, remote_folder.name.clone()).await?
        };

        let commits = match requests::fetch_commits(remote_folder.id()).await {
            Ok(commits) => commits,
            Err(e) => {
                log::debug!("failed to fetch commits for folder with remote id {}, {e:?}", remote_folder.id);

                continue
            }
        };

        for commit in commits {
            let note = storage::fetch_note_by_remote_id(&mut conn, commit.note_id(), folder.local_id()).await?;

            if let Some(note) = note {
                if ModelState::Clean == note.state && note.commit_id < commit.commit_id {
                    // A note fetched by its remote id must have remote id. Hence we can safely unwrap it
                    match requests::fetch_note(note.remote_id().unwrap()).await {
                        Ok(remote_note) => storage::update_note(
                            &mut conn,
                            note.local_id(),
                            remote_note.title.as_ref().map(|title| title.as_str()),
                            remote_note.text.as_str(),
                            remote_note.commit_id,
                            ModelState::Clean,
                        ).await?,
                        Err(e) => log::debug!("failed to fetch note with remote id {}, {e:?}", note.remote_id().unwrap().0),
                    }
                }
            } else {
                match requests::fetch_note(commit.note_id()).await {
                    Ok(note) => {
                        storage::create_note(&mut conn, folder.local_id(), Some(note.id()), note.title, note.text, note.commit_id).await?;
                    },
                    Err(e) => log::debug!("failed to fetch note with remote id {}, {e:?}", commit.note_id),
                };
            }
        }
    }

    let local_folders = storage::fetch_account_folders(&mut conn, mavinote_account.id).await?
        .into_iter()
        .filter(|folder| folder.account_id == mavinote_account.id)
        .collect::<Vec<Folder>>();

    for local_folder in local_folders {
        if let ModelState::Deleted = local_folder.state {
            if let Some(remote_id) = local_folder.remote_id() {
                if let Ok(_) = requests::delete_folder(remote_id).await {
                    storage::delete_folder(&mut conn, local_folder.local_id()).await?;
                }
            } else {
                log::error!("A folder cannot be in deleted state while being not created at remote side");
            }

            continue;
        }

        let mut remote_folder_id = local_folder.remote_id();

        if remote_folder_id.is_none() {
            match requests::create_folder(local_folder.name.as_str()).await {
                Ok(remote_folder) => {
                    sqlx::query("update folders set remote_id = ? where id = ?")
                        .bind(remote_folder.id)
                        .bind(local_folder.id)
                        .execute(&mut conn)
                        .await?;

                    remote_folder_id = Some(remote_folder.id());
                },
                Err(e) => log::error!("failed to create local folder in remote, {e:?}"),
            }
        };

        if let Some(remote_folder_id) = remote_folder_id {
            let local_notes = storage::fetch_all_notes(&mut conn, local_folder.local_id()).await?;

            for local_note in local_notes {
                if let ModelState::Deleted = local_note.state {
                    if let Some(remote_id) = local_note.remote_id() {
                        if let Ok(_) = requests::delete_note(remote_id).await {
                            storage::delete_note(&mut conn, local_note.local_id()).await?;
                        }
                    } else {
                        log::error!("A note cannot be in deleted state while being not created at remote side");
                    }
                } else if let Some(remote_id) = local_note.remote_id() {
                    if ModelState::Modified == local_note.state {
                        match requests::update_note(remote_id, local_note.title.as_ref().map(|title| title.as_str()), local_note.text.as_str()).await {
                            Ok(commit) => storage::update_commit(&mut conn, local_note.local_id(), commit.commit_id).await?,
                            Err(e) => log::debug!("failed to update note with id {}, {e:?}", local_note.id),
                        }
                    }
                } else if local_note.remote_id().is_none() {
                    match requests::create_note(remote_folder_id, local_note.title.as_ref().map(|title| title.as_str()), local_note.text.as_str()).await {
                        Ok(remote_note) => {
                            sqlx::query("update notes set remote_id = ?, commit_id = ? where id = ?")
                                .bind(remote_note.id)
                                .bind(remote_note.commit_id)
                                .bind(local_note.id)
                                .execute(&mut conn)
                                .await?;
                        },
                        Err(e) => log::debug!("failed to create local note in remote, {e:?}"),
                    }
                }
            }
        }
    }

    FOLDERS.get().unwrap().send_replace(storage::fetch_folders(&mut conn).await.into());

    Ok(())
}

pub async fn sync() -> Result<(), Error> {
    start_sync();
    let res = try_sync().await;
    end_sync();

    res
}

pub async fn accounts() -> tokio::sync::watch::Receiver<State<Vec<Account>, Error>> {
    let sender = ACCOUNTS.get().unwrap();
    let load = match *sender.borrow() {
        State::Initial | State::Err(_) => true,
        _ => false,
    };

    if load {
        sender.send_replace(State::Loading);

        let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();

        sender.send_replace(storage::fetch_accounts(&mut conn).await.into());
    }

    sender.subscribe()

}

pub async fn folders() -> tokio::sync::watch::Receiver<State<Vec<Folder>, Error>> {
    let sender = FOLDERS.get().unwrap();
    let load = match *sender.borrow() {
        State::Initial | State::Err(_) => true,
        _ => false,
    };

    if load {
        sender.send_replace(State::Loading);

        let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();

        sender.send_replace(storage::fetch_folders(&mut conn).await.into());
    }

    sender.subscribe()
}

pub async fn folder(folder_id: i32) -> Result<Option<Folder>, Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();
    storage::fetch_folder(&mut conn, LocalId(folder_id)).await
}

pub async fn create_folder(account_id: i32, name: String) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let account = storage::fetch_account(&mut conn, account_id)
        .await?
        .ok_or_else(|| {
            log::error!("trying to create a folder in an unknown account, {account_id}");

            return Error::Message("unknown account".to_string());
        })?;

    let remote_id = if account.kind == AccountKind::Mavinote {
        match requests::create_folder(name.as_str()).await {
            Ok(folder) => Some(folder.id()),
            Err(e) => {
                log::debug!("failed to create folder in remote {e:?}");
                None
            },
        }
    } else {
        None
    };

    let folder = storage::create_folder(&mut conn, remote_id, account_id, name).await?;

    FOLDERS.get().unwrap().send_modify(move |state| {
        if let State::Ok(folders) = state {
            folders.push(folder);
        }
    });

    Ok(())
}

pub async fn delete_folder(folder_id: i32) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let folder = if let Some(folder) = storage::fetch_folder(&mut conn, LocalId(folder_id)).await? {
        folder
    } else {
        log::error!("trying to delete folder with id {folder_id} which does not exist in storage");

        return Ok(());
    };

    let mut delete = true;

    if let Some(remote_id) = folder.remote_id() {
        if let Err(e) = requests::delete_folder(remote_id).await {
            log::debug!("failed to delete folder in remote, {e:?}");

            delete = false;
        }
    }

    if delete {
        storage::delete_folder(&mut conn, folder.local_id()).await?;
    } else {
        storage::delete_folder_local(&mut conn, folder.local_id()).await?;
    }

    FOLDERS.get().unwrap().send_replace(storage::fetch_folders(&mut conn).await.into());

    Ok(())
}

pub async fn notes(folder_id: i32) -> Receiver<State<Vec<Note>, Error>> {
    let notes_map = NOTES_MAP.get().unwrap();

    if !notes_map.contains_key(folder_id) {
        notes_map.insert(folder_id, State::Loading);

        let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();
        notes_map.update(folder_id, storage::fetch_notes(&mut conn, LocalId(folder_id)).await.into());
    }

    notes_map.subscribe(folder_id).unwrap()
}

pub async fn note(note_id: i32) -> Result<Option<Note>, Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();
    storage::fetch_note(&mut conn, LocalId(note_id)).await
}

pub async fn create_note(folder_id: i32) -> Result<i32, Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();

    let folder = storage::fetch_folder(&mut conn, LocalId(folder_id))
        .await?
        .ok_or_else(|| {
            log::error!("trying to create a note in a folder with id {folder_id} which does not exist");

            return Error::Message("unknown folder".to_string());
        })?;

    let account = storage::fetch_account(&mut conn, folder.account_id)
        .await?
        .ok_or_else(|| {
            log::error!("a folder with unknown account id {} cannot be exist", folder.account_id);

            return Error::Message("unknown account".to_string());
        })?;


    let title = Some("".to_string());
    let text = "".to_string();

    let remote_note = if account.kind == AccountKind::Mavinote {
        if let Some(remote_id) = folder.remote_id() {
            match requests::create_note(remote_id, title.as_ref().map(|title| title.as_str()), text.as_str()).await {
                Ok(note) => Some(note),
                Err(e) => {
                    log::debug!("failed to create note in remote, {e:?}");

                    None
                },
            }
        } else {
            None
        }
    } else {
        None
    };

    let local_note = storage::create_note(
        &mut conn,
        folder.local_id(),
        remote_note.as_ref().map(|n| n.id()),
        title,
        text,
        remote_note.map(|n| n.commit_id).unwrap_or(0)
    ).await?;

    let note_id = local_note.id;

    NOTES_MAP.get().unwrap().update_modify(folder_id, move |state| {
        if let State::Ok(notes) = state {
            notes.push(local_note);
        }
    });

    Ok(note_id)
}

pub async fn update_note(note_id: i32, text: String) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let note = match storage::fetch_note(&mut conn, LocalId(note_id)).await? {
        Some(note) => note,
        None => {
            log::error!("trying to update a note with id {note_id} which does not exist");

            return Ok(());
        }
    };

    let text = text.as_str().trim();
    let ending_index = text.char_indices().nth(30).unwrap_or((text.len(), ' ')).0;
    let title = text[..ending_index].replace('\n', "");

    let (commit_id, state) = if let Some(remote_id) = note.remote_id() {
        match requests::update_note(remote_id, Some(title.as_str()), text).await {
            Ok(commit) => (commit.commit_id, ModelState::Clean),
            Err(e) => {
                log::debug!("failed to update note with id {note_id}, {e:?}");
                (note.commit_id, ModelState::Modified)
            }
        }
    } else {
        (note.commit_id, ModelState::Clean)
    };

    storage::update_note(&mut conn, note.local_id(), Some(title.as_str()), text, commit_id, state).await?;

    if let Some(updated_note) = storage::fetch_note(&mut conn, note.local_id()).await? {
        NOTES_MAP.get().unwrap().update_modify(note.folder_id, move |state| {
            if let State::Ok(notes) = state {
                if let Some(note) = notes.iter_mut().find(|n| n.id == note_id) {
                    *note = updated_note;
                }
            }
        });
    }

    Ok(())
}

pub async fn delete_note(note_id: i32) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let note = if let Some(note) = storage::fetch_note(&mut conn, LocalId(note_id)).await? {
        note
    } else {
        log::error!("trying to delete note with id {note_id} which does not exist in storage");

        return Ok(());
    };

    let mut delete = true;

    if let Some(remote_id) = note.remote_id() {
        if let Err(e) = requests::delete_note(remote_id).await {
            log::debug!("failed to delete note in remote, {e:?}");

            delete = false;
        }
    }

    if delete {
        storage::delete_note(&mut conn, note.local_id()).await?;
    } else {
        storage::delete_note_local(&mut conn, note.local_id()).await?;
    }

    Ok(())

}
