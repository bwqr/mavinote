use std::sync::Arc;

use account::{Account, Mavinote, Folder, Note, State as ModelState};
use base::{State, Error, observable_map::{ObservableMap, Receiver}};
use futures::{stream::{AbortHandle, Abortable}, FutureExt};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use once_cell::sync::OnceCell;
use tokio::sync::watch::{channel, Sender};
use wasm_bindgen_futures::spawn_local;

use crate::{serialize_to_buffer, send_stream, Message};

static FOLDERS: OnceCell<Sender<State<Vec<Folder>, Error>>> = OnceCell::new();
static NOTES_MAP: OnceCell<Arc<ObservableMap<State<Vec<Note>, Error>>>> = OnceCell::new();

pub fn init() {
    FOLDERS.set(channel(State::Initial).0).unwrap();
    NOTES_MAP.set(Arc::new(ObservableMap::new())).unwrap();
}

pub async fn folders() -> tokio::sync::watch::Receiver<State<Vec<Folder>, Error>> {
    let sender = FOLDERS.get().unwrap();
    let load = match *sender.borrow() {
        State::Initial | State::Err(_) => true,
        _ => false,
    };

    if load {
        sender.send_replace(State::Loading);

        let mavinote = runtime::get::<Mavinote>().unwrap();
        let folders = mavinote.fetch_folders()
            .await
            .map(|vec| vec.into_iter().filter(|f| if let ModelState::Clean = f.state { true } else { false }).collect())
            .map_err(|e| e.into());

        sender.send_replace(folders.into());
    }

    sender.subscribe()
}

#[wasm_bindgen]
pub fn note_folders(stream_id: u32) -> *mut AbortHandle {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let future = Abortable::new(async move {
        let mut rx = folders().await;

        match &*rx.borrow() {
            State::Ok(ok) => send_stream(stream_id, Message::Ok(ok)),
            State::Err(e) => send_stream::<Error>(stream_id, Message::Err(e.clone())),
            _ => {},
        };

        while rx.changed().await.is_ok() {
            match &*rx.borrow() {
                State::Ok(ok) => send_stream(stream_id, Message::Ok(ok)),
                State::Err(e) => send_stream::<Error>(stream_id, Message::Err(e.clone())),
                _ => {},
            };
        }

        send_stream::<Vec<Folder>>(stream_id, Message::Complete);

    }, abort_registration);

    spawn_local(future.map(|_| ()));

    Box::into_raw(Box::new(abort_handle))
}

#[wasm_bindgen]
pub async fn note_create_folder(folder_name: String) -> Result<Uint8Array, Uint8Array> {
    let mavinote = runtime::get::<Mavinote>().unwrap();

    mavinote.create_folder(folder_name).await
        .map(serialize_to_buffer)
        .map_err(serialize_to_buffer)
}

#[wasm_bindgen]
pub async fn note_delete_folder(folder_id: i32) -> Result<Uint8Array, Uint8Array> {
    let mavinote = runtime::get::<Mavinote>().unwrap();

    mavinote.delete_folder(folder_id).await
        .map(serialize_to_buffer)
        .map_err(serialize_to_buffer)
}

pub async fn notes(folder_id: i32) -> Receiver<State<Vec<Note>, Error>> {
        let notes_map = NOTES_MAP.get().unwrap();

        if !notes_map.contains_key(folder_id) {
            notes_map.insert(folder_id, State::Loading);

            let mavinote = runtime::get::<Mavinote>().unwrap();
            let notes = mavinote.fetch_notes(folder_id)
                .await
                .map(|vec| vec.into_iter().filter(|n| if let ModelState::Clean = n.state { true } else { false }).collect())
                .map_err(|e| e.into());

            notes_map.update(folder_id, notes.into());
        }

        notes_map.subscribe(folder_id).unwrap()
}

#[wasm_bindgen]
pub fn note_notes(stream_id: u32, folder_id: i32) -> *mut AbortHandle {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let future = Abortable::new(async move {
        let mut rx = notes(folder_id).await;

        match &*rx.inner().borrow() {
            State::Ok(ok) => send_stream(stream_id, Message::Ok(ok)),
            State::Err(e) => send_stream::<Error>(stream_id, Message::Err(e.clone())),
            _ => {},
        };

        while rx.inner().changed().await.is_ok() {
            match &*rx.inner().borrow() {
                State::Ok(ok) => send_stream(stream_id, Message::Ok(ok)),
                State::Err(e) => send_stream::<Error>(stream_id, Message::Err(e.clone())),
                _ => {},
            };
        }

        send_stream::<Vec<Note>>(stream_id, Message::Complete);
    }, abort_registration);

    spawn_local(future.map(|_| ()));

    Box::into_raw(Box::new(abort_handle))
}

#[wasm_bindgen]
pub async fn note_note(note_id: i32) -> Result<Uint8Array, Uint8Array> {
    let mavinote = runtime::get::<Mavinote>().unwrap();

    mavinote.fetch_note(note_id).await
        .map(serialize_to_buffer)
        .map_err(serialize_to_buffer)
}

#[wasm_bindgen]
pub async fn note_create_note(folder_id: i32, text: String) -> Result<Uint8Array, Uint8Array> {
    let mavinote = runtime::get::<Mavinote>().unwrap();

    let text = text.as_str().trim();
    let ending_index = text.char_indices().nth(30).unwrap_or((text.len(), ' ')).0;
    let title = text[..ending_index].replace('\n', "");

    let note = mavinote.create_note(folder_id, Some(title.as_str()), &text)
        .await
        .map_err(serialize_to_buffer)?;

    let buffer = serialize_to_buffer(&note);

    NOTES_MAP.get().unwrap().update_modify(folder_id, move |state| {
        if let State::Ok(vec) = state {
            vec.push(note);
        }
    });
    
    Ok(buffer)
}

#[wasm_bindgen]
pub async fn note_update_note(folder_id: i32, note_id: i32, text: String) -> Result<Uint8Array, Uint8Array> {
    let mavinote = runtime::get::<Mavinote>().unwrap();

    let text = text.as_str().trim();
    let ending_index = text.char_indices().nth(30).unwrap_or((text.len(), ' ')).0;
    let title = text[..ending_index].replace('\n', "");

    let res = mavinote.update_note(note_id, Some(title.as_str()), &text).await
        .map_err(serialize_to_buffer)?;

    NOTES_MAP.get().unwrap().update_modify(folder_id, move |state| {
        if let State::Ok(vec) = state {
            if let Some(n) = vec.into_iter().find(|n| n.id == note_id) {
                n.title = Some(title);
                n.text = text.to_string();
            }
        }
    });

    Ok(serialize_to_buffer(res))
}

#[wasm_bindgen]
pub async fn note_delete_note(folder_id: i32, note_id: i32) -> Result<Uint8Array, Uint8Array> {
    let mavinote = runtime::get::<Mavinote>().unwrap();

    let res = mavinote.delete_note(note_id).await
        .map_err(serialize_to_buffer)?;

    NOTES_MAP.get().unwrap().update_modify(folder_id, move |state| {
        if let State::Ok(vec) = state {
            let index = vec.iter().position(|n| n.id == note_id);
            if let Some(index) = index {
                vec.remove(index);
            }
        }
    });

    Ok(serialize_to_buffer(res))
}
