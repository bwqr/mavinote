pub mod models;

use std::sync::Arc;

use once_cell::sync::OnceCell;
use sqlx::{Pool, Sqlite};
use tokio::sync::watch::{channel, Sender};

use base::{Error, State, observable_map::{ObservableMap, Receiver}};

use models::{Folder, Note, NoteState};

mod requests;
mod responses;
mod storage;

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
    FOLDERS.set(channel(State::default()).0).unwrap();
    NOTES_MAP.set(Arc::new(ObservableMap::new())).unwrap();
    ACTIVE_SYNCS.set(channel(0).0).unwrap();
}

pub fn active_syncs() -> tokio::sync::watch::Receiver<i32> {
    ACTIVE_SYNCS.get().unwrap().subscribe()
}

pub async fn sync() -> Result<(), Error> {
    start_sync();

    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await?;

    let folders = requests::fetch_folders().await?;

    for f in folders {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        let folder = storage::fetch_folder(&mut conn, f.id).await?;

        if let Some(folder) = folder {
            let commits = requests::fetch_commits(folder.id).await?;

            for commit in commits {
                let note = storage::fetch_note(&mut conn, commit.note_id)
                    .await?;

                match note {
                    Some(note) => {
                        if note.state == NoteState::Clean && note.commit_id < commit.commit_id {
                            log::debug!("note {} needs to be pulled", note.id);
                            if let Some(note) = requests::fetch_note(commit.note_id).await? {
                                let note = note.into();

                                storage::update_note(&mut conn, &note).await?;

                                NOTES_MAP.get().unwrap().update_modify(folder.id, |state| {
                                    if let State::Ok(notes) = state {
                                        if let Some(n) = &mut notes.iter_mut().find(|n| n.id == note.id) {
                                            **n = note;
                                        }
                                    }
                                });
                            }
                        } else if note.state != NoteState::Clean && note.commit_id == commit.commit_id {
                            log::debug!("note {} needs to be pushed", note.id);
                        } else if note.state != NoteState::Clean && note.commit_id < commit.commit_id {
                            log::debug!("note {} needs to be synced", note.id);
                        } else {
                            log::debug!("note {} is up to date", note.id);
                        }
                    },
                    None => {
                        log::debug!("note {} needs to be created", commit.note_id);

                        if let Some(note) = requests::fetch_note(commit.note_id).await? {
                            let note = note.into();

                            storage::create_note(&mut conn, &note).await?;

                            NOTES_MAP.get().unwrap().update_modify(folder.id, |state| {
                                if let State::Ok(notes) = state {
                                    notes.push(note);
                                }
                            });
                        }
                    }
                }
            }
        } else {
            storage::create_folder(&mut conn, f).await?;
            FOLDERS.get().unwrap().send_replace(storage::fetch_folders(&mut conn).await.into());
        }
    }

    end_sync();

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
        sender.send_replace(storage::fetch_folders(&mut conn).await.into());
    }

    sender.subscribe()
}

pub async fn create_folder(name: String) -> Result<(), Error> {
    let folder = requests::create_folder(name).await?;

    FOLDERS.get().unwrap().send_modify(move |state| {
        if let State::Ok(folders) = state {
            folders.push(folder);
        }
    });

    Ok(())
}

pub async fn notes(folder_id: i32) -> Receiver<State<Vec<Note>, Error>> {
    let notes_map = NOTES_MAP.get().unwrap();

    if !notes_map.contains_key(folder_id) {
        notes_map.insert(folder_id, State::Loading);

        let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();
        notes_map.update(folder_id, storage::fetch_notes(&mut conn, folder_id).await.into());
    }

    notes_map.subscribe(folder_id).unwrap()
}

pub async fn note(note_id: i32) -> Result<Option<Note>, Error> {
    let mut conn = runtime::get::<Arc<Pool<Sqlite>>>().unwrap().acquire().await.unwrap();
    storage::fetch_note(&mut conn, note_id).await
}

pub async fn create_note(folder_id: i32) -> Result<i32, Error> {
    start_sync();

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let note = requests::create_note(folder_id).await?;
    let note_id = note.id;

    NOTES_MAP.get().unwrap().update_modify(folder_id, move |state| {
        if let State::Ok(notes) = state {
            notes.push(note.into());
        }
    });

    end_sync();

    Ok(note_id)
}

pub async fn update_note(note_id: i32, folder_id: i32, text: String) -> Result<(), Error> {
    start_sync();

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let text = text.as_str().trim().to_string();
    let ending_index = text.char_indices().nth(30).unwrap_or((text.len(), ' ')).0;
    let title = text.as_str()[..ending_index].replace('\n', "");

    requests::update_note(note_id, &title, &text).await?;

    NOTES_MAP.get().unwrap().update_modify(folder_id, move |state| {
        if let State::Ok(notes) = state {
            if let Some(mut note) = notes.iter_mut().find(|n| n.id == note_id) {
                note.text = text;
                note.title = Some(title);
            }
        }
    });

    end_sync();

    Ok(())
}
