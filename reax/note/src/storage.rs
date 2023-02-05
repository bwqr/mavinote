use std::sync::Arc;


use once_cell::sync::OnceCell;
use sqlx::{Pool, Sqlite, types::Json};
use tokio::sync::watch::{channel, Sender};

use base::{State, observable_map::{ObservableMap, Receiver}, Config};

use crate::{Error, StorageError};
use crate::accounts::mavinote::{MavinoteClient, CreateFolderRequest, CreateNoteRequest};
use crate::models::{Folder, Note, State as ModelState, LocalId, Account, AccountKind, Mavinote};


pub mod db;
pub mod sync;

static ACCOUNTS: OnceCell<Sender<State<Vec<Account>, Error>>> = OnceCell::new();
pub(crate) static FOLDERS: OnceCell<Sender<State<Vec<Folder>, Error>>> = OnceCell::new();
static NOTES_MAP: OnceCell<Arc<ObservableMap<State<Vec<Note>, Error>>>> = OnceCell::new();

pub async fn init() -> Result<(), Error> {
    ACCOUNTS.set(channel(State::default()).0).unwrap();
    FOLDERS.set(channel(State::default()).0).unwrap();
    NOTES_MAP.set(Arc::new(ObservableMap::new())).unwrap();

    Ok(())
}

pub(crate) async fn mavinote_client(account_id: i32) -> Result<Option<MavinoteClient>, Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    db::fetch_account_data::<Mavinote>(&mut conn, account_id).await
        .map(|opt| opt.map(|mavinote| MavinoteClient::new(Some(account_id), config.api_url.clone(), Some(mavinote.token))))
        .map_err(|e| e.into())
}

pub async fn send_code(email: String) -> Result<(), Error> {
    let config = runtime::get::<Arc<Config>>().unwrap();

    MavinoteClient::new(None, config.api_url.clone(), None)
        .send_code(&email).await
        .map_err(|e| e.into())
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

        sender.send_replace(db::fetch_accounts(&mut conn).await.map_err(|e| e.into()).into());
    }

    sender.subscribe()
}

pub async fn account(account_id: i32) -> Result<Option<Account>, Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();
    db::fetch_account(&mut conn, account_id).await.map_err(|e| e.into())
}

pub async fn mavinote_account(account_id: i32) -> Result<Option<Mavinote>, Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();
    db::fetch_account_data::<Mavinote>(&mut conn, account_id).await.map_err(|e| e.into())
}

pub async fn sign_up(name: String, email: String, code: String) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    if db::account_with_name_exists(&mut conn, &email).await? {
        return Err(Error::Storage(StorageError::AccountNameUsed));
    }

    let config = runtime::get::<Arc<Config>>().unwrap();

    let token = MavinoteClient::new(None, config.api_url.clone(), None)
        .sign_up(&email, &code)
        .await?;

    db::create_account(&mut conn, name, AccountKind::Mavinote, Some(Json(Mavinote { email, token: token.token }))).await?;

    ACCOUNTS.get().unwrap().send_replace(db::fetch_accounts(&mut conn).await.map_err(|e| e.into()).into());

    Ok(())
}

pub async fn delete_account(account_id: i32) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let account = db::fetch_account(&mut conn, account_id).await?
        .ok_or_else(|| {
            log::error!("trying to delete an unknown account, {account_id}");

            Error::Storage(StorageError::AccountNotFound)
        })?;

    if account.kind != AccountKind::Mavinote {
        log::error!("can delete only mavinote account");

        return Err(Error::Storage(StorageError::NotMavinoteAccount));
    }

    db::delete_account(&mut conn, account_id).await?;

    ACCOUNTS.get().unwrap().send_replace(db::fetch_accounts(&mut conn).await.map_err(|e| e.into()).into());
    FOLDERS.get().unwrap().send_replace(db::fetch_folders(&mut conn).await.map_err(|e| e.into()).into());

    Ok(())
}

pub async fn add_device(account_id: i32, fingerprint: String) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let account = db::fetch_account(&mut conn, account_id)
        .await?
        .ok_or(Error::Storage(StorageError::AccountNotFound))?;

    if account.kind != AccountKind::Mavinote {
        return Err(Error::Storage(StorageError::NotMavinoteAccount));
    }

    let mavinote = mavinote_client(account_id).await?.unwrap();

    let device_id = mavinote.add_device(fingerprint).await?;

    db::create_devices(&mut conn, account_id, &[device_id]).await?;

    Ok(())
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

        sender.send_replace(db::fetch_folders(&mut conn).await.map_err(|e| e.into()).into());
    }

    sender.subscribe()
}

pub async fn folder(folder_id: i32) -> Result<Option<Folder>, Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();
    db::fetch_folder(&mut conn, LocalId(folder_id)).await.map_err(|e| e.into())
}

pub async fn create_folder(account_id: i32, name: String) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let account = db::fetch_account(&mut conn, account_id)
        .await?
        .ok_or_else(|| {
            log::error!("trying to create a folder in an unknown account, {account_id}");

            return Error::Storage(StorageError::AccountNotFound);
        })?;

    let remote_id = if account.kind == AccountKind::Mavinote {
        let mavinote = mavinote_client(account.id).await?.unwrap();

        let devices = db::fetch_devices(&mut conn, account.id).await?;

        let device_folders = devices
            .into_iter()
            .map(|device| CreateFolderRequest{ device_id: device.id, name: &name })
            .collect();

        match mavinote.create_folder(device_folders).await {
            Ok(folder) => Some(folder.id()),
            Err(e) => {
                log::error!("failed to create folder in remote {e:?}");
                None
            }
        }
    } else {
        None
    };

    let folder = db::create_folder(&mut conn, remote_id, account_id, name).await?;

    FOLDERS.get().unwrap().send_modify(move |state| {
        if let State::Ok(folders) = state {
            folders.push(folder);
        }
    });

    Ok(())
}

pub async fn delete_folder(folder_id: i32) -> Result<(), Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let folder = if let Some(folder) = db::fetch_folder(&mut conn, LocalId(folder_id)).await? {
        folder
    } else {
        log::error!("trying to delete folder with id {folder_id} which does not exist in storage");

        return Ok(());
    };

    let mut delete = true;

    if let Some(remote_id) = folder.remote_id() {
        let account = db::fetch_account(&mut conn, folder.account_id).await?.unwrap();
        if let AccountKind::Mavinote = account.kind {
            let mavinote = mavinote_client(account.id).await?.unwrap();

            if let Err(e) = mavinote.delete_folder(remote_id).await {
                log::debug!("failed to delete folder in remote, {e:?}");

                delete = false;
            }
        }
    }

    if delete {
        db::delete_folder(&mut conn, folder.local_id()).await?;
    } else {
        db::delete_folder_local(&mut conn, folder.local_id()).await?;
    }

    FOLDERS.get().unwrap().send_replace(db::fetch_folders(&mut conn).await.map_err(|e| e.into()).into());

    Ok(())
}

pub async fn notes(folder_id: i32) -> Receiver<State<Vec<Note>, Error>> {
    let notes_map = NOTES_MAP.get().unwrap();

    if !notes_map.contains_key(folder_id) {
        notes_map.insert(folder_id, State::Loading);

        let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();
        notes_map.update(folder_id, db::fetch_notes(&mut conn, LocalId(folder_id)).await.map_err(|e| e.into()).into());
    }

    notes_map.subscribe(folder_id).unwrap()
}

pub async fn note(note_id: i32) -> Result<Option<Note>, Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();
    db::fetch_note(&mut conn, LocalId(note_id)).await.map_err(|e| e.into())
}

pub async fn create_note(folder_id: i32, text: String) -> Result<i32, Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();

    let folder = db::fetch_folder(&mut conn, LocalId(folder_id))
        .await?
        .ok_or_else(|| {
            log::error!("trying to create a note in a folder with id {folder_id} which does not exist");

            return Error::Storage(StorageError::FolderNotFound);
        })?;

    let account = db::fetch_account(&mut conn, folder.account_id)
        .await?
        .ok_or_else(|| {
            log::error!("a folder with unknown account id {} cannot be exist", folder.account_id);

            return Error::Storage(StorageError::AccountNotFound);
        })?;


    let text = text.as_str().trim();
    let ending_index = text.char_indices().nth(30).unwrap_or((text.len(), ' ')).0;
    let name = text[..ending_index].replace('\n', "");

    let remote_note = if account.kind == AccountKind::Mavinote {
        if let Some(remote_id) = folder.remote_id() {
            let mavinote = mavinote_client(account.id).await?.unwrap();

            let devices = db::fetch_devices(&mut conn, account.id).await?;
            let device_notes = devices
                .into_iter()
                .map(|device| CreateNoteRequest{ device_id: device.id, name: &name, text: &text })
                .collect();

            match mavinote.create_note(remote_id, device_notes).await {
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

    let local_note = db::create_note(
        &mut conn,
        folder.local_id(),
        remote_note.as_ref().map(|n| n.id()),
        name,
        text.to_string(),
        remote_note.map(|n| n.commit).unwrap_or(0)
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

    let note = match db::fetch_note(&mut conn, LocalId(note_id)).await? {
        Some(note) => note,
        None => {
            log::error!("trying to update a note with id {note_id} which does not exist");

            return Ok(());
        }
    };

    let text = text.as_str().trim();
    let ending_index = text.char_indices().nth(30).unwrap_or((text.len(), ' ')).0;
    let name = text[..ending_index].replace('\n', "");

    let (commit, state) = if let Some(remote_id) = note.remote_id() {
        let folder = db::fetch_folder(&mut conn, LocalId(note.folder_id)).await?.unwrap();
        let account = db::fetch_account(&mut conn, folder.account_id).await?.unwrap();
        if let AccountKind::Mavinote = account.kind {
            let mavinote = mavinote_client(account.id).await?.unwrap();

            let devices = db::fetch_devices(&mut conn, account.id).await?;
            let device_notes = devices
                .into_iter()
                .map(|device| CreateNoteRequest{ device_id: device.id, name: &name, text: &text })
                .collect();

            match mavinote.update_note(remote_id, note.commit, device_notes).await {
                Ok(commit) => (commit.commit, ModelState::Clean),
                Err(e) => {
                    log::debug!("failed to update note with id {note_id}, {e:?}");
                    (note.commit, ModelState::Modified)
                }
            }
        } else {
            (note.commit, ModelState::Clean)
        }
    } else {
        (note.commit, ModelState::Clean)
    };

    db::update_note(&mut conn, note.local_id(), &name, text, commit, state).await?;

    if let Some(updated_note) = db::fetch_note(&mut conn, note.local_id()).await? {
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

    let note = if let Some(note) = db::fetch_note(&mut conn, LocalId(note_id)).await? {
        note
    } else {
        log::error!("trying to delete note with id {note_id} which does not exist in storage");

        return Ok(());
    };

    let mut delete = true;

    if let Some(remote_id) = note.remote_id() {
        let folder = db::fetch_folder(&mut conn, LocalId(note.folder_id)).await?.unwrap();
        let account = db::fetch_account(&mut conn, folder.account_id).await?.unwrap();
        if let AccountKind::Mavinote = account.kind {
            let mavinote = mavinote_client(account.id).await?.unwrap();

            if let Err(e) = mavinote.delete_note(remote_id).await {
                log::debug!("failed to delete note in remote, {e:?}");

                delete = false;
            }
        }
    }

    if delete {
        db::delete_note(&mut conn, note.local_id()).await?;
    } else {
        db::delete_note_local(&mut conn, note.local_id()).await?;
    }

    Ok(())
}
