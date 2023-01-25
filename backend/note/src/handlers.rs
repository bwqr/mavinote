use actix_web::{
    delete, get, post, put,
    web::{block, Data, Json, Path},
};
use diesel::prelude::*;

use base::{
    sanitize::Sanitized,
    schema::{device_folders, device_notes, devices, folders, notes},
    types::Pool,
    HttpError, HttpMessage,
};
use user::models::Device;

use crate::{
    models::{Folder, Note, State},
    requests::{CreateFolderRequest, CreateNoteRequest, UpdateNoteRequest},
    responses::{self, Commit, CreatedFolder, CreatedNote},
};

#[get("folders")]
pub async fn fetch_folders(
    pool: Data<Pool>,
    device: Device,
) -> Result<Json<Vec<responses::Folder>>, HttpError> {
    let folders = block(move || {
        folders::table
            .filter(folders::user_id.eq(device.user_id))
            .left_join(
                device_folders::table.on(device_folders::folder_id
                    .eq(folders::id)
                    .and(device_folders::receiver_device_id.eq(device.id))),
            )
            .select((
                folders::id,
                folders::state,
                (device_folders::sender_device_id, device_folders::name).nullable(),
            ))
            .load(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(folders))
}

#[post("folder")]
pub async fn create_folder(
    pool: Data<Pool>,
    request: Sanitized<Json<Vec<CreateFolderRequest>>>,
    device: Device,
) -> Result<Json<CreatedFolder>, HttpError> {
    let created_folder = block(move || {
        let folders_to_create = request.0 .0;

        let mut conn = pool.get().unwrap();

        let devices = devices::table
            .filter(devices::user_id.eq(device.user_id))
            .filter(devices::id.ne(device.id))
            .load::<Device>(&mut conn)?;

        let device_not_exist_in_request = folders_to_create
            .iter()
            .find(|folder_to_create| {
                devices
                    .iter()
                    .find(|d| d.id == folder_to_create.device_id)
                    .is_none()
            })
            .is_some();

        if devices.len() != folders_to_create.len() || device_not_exist_in_request {
            return Err(HttpError::unprocessable_entity("devices_mismatch"));
        }

        let folder: Folder = diesel::insert_into(folders::table)
            .values(folders::user_id.eq(device.user_id))
            .get_result(&mut conn)?;

        diesel::insert_into(device_folders::table)
            .values(
                folders_to_create
                    .into_iter()
                    .map(|folder_to_create| {
                        (
                            device_folders::folder_id.eq(folder.id),
                            device_folders::receiver_device_id.eq(folder_to_create.device_id),
                            device_folders::sender_device_id.eq(device.id),
                            device_folders::name.eq(folder_to_create.name),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .execute(&mut conn)?;

        Ok(CreatedFolder { id: folder.id })
    })
    .await??;

    Ok(Json(created_folder))
}

#[delete("folder/{folder_id}")]
pub async fn delete_folder(
    pool: Data<Pool>,
    folder_id: Path<i32>,
    device: Device,
) -> Result<Json<HttpMessage>, HttpError> {
    block(move || {
        let mut conn = pool.get().unwrap();

        let folder_id = folders::table
            .filter(folders::id.eq(folder_id.into_inner()))
            .filter(folders::state.eq(State::Clean))
            .filter(folders::user_id.eq(device.user_id))
            .select(folders::id)
            .first::<i32>(&mut conn)?;

        diesel::update(folders::table)
            .filter(folders::id.eq(folder_id))
            .set(folders::state.eq(State::Deleted))
            .execute(&mut conn)?;

        diesel::delete(notes::table)
            .filter(notes::folder_id.eq(folder_id))
            .execute(&mut conn)?;

        diesel::delete(device_folders::table)
            .filter(device_folders::folder_id.eq(folder_id))
            .execute(&mut conn)
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}

#[get("folder/{folder_id}/commits")]
pub async fn fetch_commits(
    pool: Data<Pool>,
    folder_id: Path<i32>,
    device: Device,
) -> Result<Json<Vec<Commit>>, HttpError> {
    let commits = block(move || {
        let mut conn = pool.get().unwrap();

        let folder_id = folders::table
            .filter(folders::id.eq(folder_id.into_inner()))
            .filter(folders::state.eq(State::Clean))
            .filter(folders::user_id.eq(device.user_id))
            .select(folders::id)
            .first::<i32>(&mut conn)?;

        notes::table
            .filter(notes::folder_id.eq(folder_id))
            .order(notes::id.desc())
            .select((notes::id, notes::commit, notes::state))
            .load(&mut conn)
    })
    .await??;

    Ok(Json(commits))
}

#[post("/folder/{folder_id}/note")]
pub async fn create_note(
    pool: Data<Pool>,
    folder_id: Path<i32>,
    request: Sanitized<Json<Vec<CreateNoteRequest>>>,
    device: Device,
) -> Result<Json<CreatedNote>, HttpError> {
    let note = block(move || {
        let notes_to_create = request.0 .0;

        let mut conn = pool.get().unwrap();

        let folder_id = folders::table
            .filter(folders::id.eq(folder_id.into_inner()))
            .filter(folders::state.eq(State::Clean))
            .filter(folders::user_id.eq(device.user_id))
            .select(folders::id)
            .first::<i32>(&mut conn)?;

        let devices = devices::table
            .filter(devices::user_id.eq(device.user_id))
            .filter(devices::id.ne(device.id))
            .load::<Device>(&mut conn)?;

        let device_not_exist_in_request = notes_to_create
            .iter()
            .find(|note_to_create| {
                devices
                    .iter()
                    .find(|d| d.id == note_to_create.device_id)
                    .is_none()
            })
            .is_some();

        if devices.len() != notes_to_create.len() || device_not_exist_in_request {
            return Err(HttpError::unprocessable_entity("devices_mismatch"));
        }

        let note: Note = diesel::insert_into(notes::table)
            .values((notes::folder_id.eq(folder_id),))
            .get_result(&mut conn)?;

        diesel::insert_into(device_notes::table)
            .values(
                notes_to_create
                    .into_iter()
                    .map(|note_to_create| {
                        (
                            device_notes::note_id.eq(note.id),
                            device_notes::receiver_device_id.eq(note_to_create.device_id),
                            device_notes::sender_device_id.eq(device.id),
                            device_notes::title.eq(note_to_create.title),
                            device_notes::text.eq(note_to_create.text),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .execute(&mut conn)?;

        Ok(CreatedNote {
            id: note.id,
            commit: note.commit,
        })
    })
    .await??;

    Ok(Json(note))
}

#[get("/note/{note_id}")]
pub async fn fetch_note(
    pool: Data<Pool>,
    note_id: Path<i32>,
    device: Device,
) -> Result<Json<responses::Note>, HttpError> {
    let note = block(move || {
        notes::table
            .filter(notes::id.eq(note_id.into_inner()))
            .filter(folders::user_id.eq(device.user_id))
            .inner_join(folders::table)
            .left_join(
                device_notes::table.on(device_notes::note_id
                    .eq(notes::id)
                    .and(device_notes::receiver_device_id.eq(device.id))),
            )
            .select((
                notes::id,
                notes::commit,
                notes::state,
                (
                    device_notes::sender_device_id,
                    device_notes::title.nullable(),
                    device_notes::text,
                )
                    .nullable(),
            ))
            .first(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(note))
}

#[put("note/{note_id}")]
pub async fn update_note(
    pool: Data<Pool>,
    note_id: Path<i32>,
    request: Sanitized<Json<UpdateNoteRequest>>,
    device: Device,
) -> Result<Json<Commit>, HttpError> {
    let commit = block(move || -> Result<Commit, HttpError> {
        let mut conn = pool.get().unwrap();

        // TODO make incrementing commit atomic
        let (note_id, commit) = notes::table
            .filter(notes::id.eq(note_id.into_inner()))
            .filter(notes::state.eq(State::Clean))
            .filter(folders::user_id.eq(device.user_id))
            .inner_join(folders::table)
            .select((notes::id, notes::commit))
            .first::<(i32, i32)>(&mut conn)?;

        if commit != request.commit {
            return Err(HttpError::conflict("commit_not_matches"));
        }

        let device_notes = request.0 .0.device_notes;

        let devices = devices::table
            .filter(devices::user_id.eq(device.user_id))
            .filter(devices::id.ne(device.id))
            .load::<Device>(&mut conn)?;

        let device_not_exist_in_request = device_notes
            .iter()
            .find(|note_to_create| {
                devices
                    .iter()
                    .find(|d| d.id == note_to_create.device_id)
                    .is_none()
            })
            .is_some();

        if devices.len() != device_notes.len() || device_not_exist_in_request {
            return Err(HttpError::unprocessable_entity("devices_mismatch"));
        }

        diesel::update(notes::table)
            .filter(notes::id.eq(note_id))
            .set(notes::commit.eq(commit + 1))
            .execute(&mut conn)?;

        // We may need to create device_note records where a device does not have a device_note
        for device_note in device_notes {
            diesel::update(device_notes::table)
                .filter(device_notes::note_id.eq(note_id))
                .filter(device_notes::receiver_device_id.eq(device_note.device_id))
                .set((
                    device_notes::sender_device_id.eq(device.id),
                    device_notes::title.eq(device_note.title),
                    device_notes::text.eq(device_note.text),
                ))
                .execute(&mut conn)?;
        }

        Ok(Commit {
            note_id,
            commit: commit + 1,
            state: State::Clean,
        })
    })
    .await??;

    Ok(Json(commit))
}

#[delete("note/{note_id}")]
pub async fn delete_note(
    pool: Data<Pool>,
    note_id: Path<i32>,
    device: Device,
) -> Result<Json<HttpMessage>, HttpError> {
    block(move || {
        let mut conn = pool.get().unwrap();

        let note_id = notes::table
            .filter(notes::id.eq(note_id.into_inner()))
            .filter(notes::state.eq(State::Clean))
            .filter(folders::user_id.eq(device.user_id))
            .inner_join(folders::table)
            .select(notes::id)
            .first::<i32>(&mut conn)?;

        diesel::update(notes::table)
            .filter(notes::id.eq(note_id))
            .set(notes::state.eq(State::Deleted))
            .execute(&mut conn)?;

        diesel::delete(device_notes::table)
            .filter(device_notes::note_id.eq(note_id))
            .execute(&mut conn)
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}
