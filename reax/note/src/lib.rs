pub mod models;

use std::sync::Arc;

use once_cell::sync::OnceCell;
use tokio::sync::watch::{channel, Sender};

use base::{Error, State, observable_map::{ObservableMap, Receiver}};

use models::{Folder, Note};

pub(crate) mod requests;


static FOLDERS: OnceCell<Sender<State<Vec<Folder>, Error>>> = OnceCell::new();
static NOTES_MAP: OnceCell<Arc<ObservableMap<State<Vec<Note>, Error>>>> = OnceCell::new();
static ACTIVE_SYNCS: OnceCell<Sender<i32>> = OnceCell::new();

pub fn init() {
    FOLDERS.set(channel(State::default()).0).unwrap();
    NOTES_MAP.set(Arc::new(ObservableMap::new())).unwrap();
    ACTIVE_SYNCS.set(channel(0).0).unwrap();
}

pub fn active_syncs() -> tokio::sync::watch::Receiver<i32> {
    ACTIVE_SYNCS.get().unwrap().subscribe()
}

pub async fn folders() -> tokio::sync::watch::Receiver<State<Vec<Folder>, Error>> {
    let sender = FOLDERS.get().unwrap();
    let load = match *sender.borrow() {
        State::Initial | State::Err(_) => true,
        _ => false,
    };

    if load {
        sender.send_replace(State::Loading);
        sender.send_replace(requests::fetch_folders().await.into());
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
        notes_map.update(folder_id, requests::fetch_notes(folder_id).await.into());
    }

    notes_map.subscribe(folder_id).unwrap()
}

pub async fn note(note_id: i32) -> Result<Option<Note>, Error> {
    requests::note(note_id).await
}

pub async fn create_note(folder_id: i32) -> Result<i32, Error> {
    let note = requests::create_note(folder_id).await?;
    let note_id = note.id;

    NOTES_MAP.get().unwrap().update_modify(folder_id, move |state| {
        if let State::Ok(notes) = state {
            notes.push(note);
        }
    });

    Ok(note_id)
}

pub async fn update_note(note_id: i32, folder_id: i32, text: String) -> Result<(), Error> {
    let text = text.as_str().trim().to_string();
    let ending_index = text.char_indices().nth(30).unwrap_or((text.len(), ' ')).0;
    let title = text.as_str()[..ending_index].replace('\n', "");

    requests::update_note(note_id, &title, &text).await?;

    NOTES_MAP.get().unwrap().update_modify(folder_id, move |state| {
        if let State::Ok(notes) = state {
            if let Some(mut note) = notes.iter_mut().find(|n| n.id == note_id) {
                note.text = title;
                note.title=  text;
            }
        }
    });

    Ok(())
}
