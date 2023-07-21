use std::collections::HashSet;

use actix_web::{
    delete, get, post, put,
    web::{block, Data, Json, Path, Query},
};
use diesel::prelude::*;

use base::{
    sanitize::Sanitized,
    schema::{
        device_folders, device_notes, folder_requests, folders, note_requests, notes, user_devices
    },
    types::Pool,
    HttpError, HttpMessage,
};
use notify::ws::messages::{SendDeviceMessage, DeviceMessage, SendExclusiveDeviceMessage};
use user::models::UserDevice;

use crate::{
    models::{Folder, Note, State},
    requests::{
        CreateFolderRequest, CreateNoteRequest, CreateRequests, FolderId, RespondRequests,
        UpdateNoteRequest,
    },
    responses::{self, Commit, CreatedFolder, CreatedNote, FolderRequest, NoteRequest, Requests, DeviceFolder},
};

#[get("folders")]
pub async fn fetch_folders(
    pool: Data<Pool>,
    device: UserDevice,
) -> Result<Json<Vec<responses::Folder>>, HttpError> {
    let folders = block(move || {
        let mut conn = pool.get().unwrap();

        let folders = folders::table
            .filter(folders::user_id.eq(device.user_id))
            .left_join(
                device_folders::table.on(device_folders::folder_id
                    .eq(folders::id)
                    .and(device_folders::receiver_device_id.eq(device.device_id))),
            )
            .select((
                folders::id,
                folders::state,
                (device_folders::sender_device_id, device_folders::name).nullable(),
            ))
            .load::<(i32, State, Option<DeviceFolder>)>(&mut conn)?;

        let commits = notes::table
            .filter(notes::folder_id.eq_any(folders.iter().map(|f| f.0)))
            .order(notes::id.desc())
            .select((notes::id, notes::folder_id, notes::commit, notes::state))
            .load::<(i32, i32, i32, State)>(&mut conn)?;

        let folders_with_commits = folders
            .into_iter()
            .map(|folder| {
                let folder_commits = commits
                    .iter()
                    .filter(|c| c.1 == folder.0)
                    .map(|c| Commit { note_id: c.0, commit: c.2, state: c.3.clone() })
                    .collect();

                responses::Folder {
                    id: folder.0,
                    state: folder.1,
                    device_folder: folder.2,
                    commits: folder_commits,
                }
            })
            .collect();

        Result::<Vec<responses::Folder>, HttpError>::Ok(folders_with_commits)
    })
    .await??;

    Ok(Json(folders))
}

pub async fn fetch_folder(
    pool: Data<Pool>,
    device: UserDevice,
    folder_id: Path<i32>,
) -> Result<Json<Option<responses::Folder>>, HttpError> {
    let folder = block(move || {
        let mut conn = pool.get().unwrap();

        let Some(folder) = folders::table
            .filter(folders::user_id.eq(device.user_id))
            .filter(folders::id.eq(folder_id.into_inner()))
            .left_join(
                device_folders::table.on(device_folders::folder_id
                    .eq(folders::id)
                    .and(device_folders::receiver_device_id.eq(device.device_id))),
            )
            .select((
                folders::id,
                folders::state,
                (device_folders::sender_device_id, device_folders::name).nullable(),
            ))
            .first::<(i32, State, Option<DeviceFolder>)>(&mut conn)
            .optional()? else {
                return Result::<Option<responses::Folder>, HttpError>::Ok(None);
            };

        let commits = notes::table
            .filter(notes::folder_id.eq(folder.0))
            .order(notes::id.desc())
            .select((notes::id, notes::commit, notes::state))
            .load::<Commit>(&mut conn)?;

        Ok(Some(responses::Folder {
            id: folder.0,
            state: folder.1,
            device_folder: folder.2,
            commits,
        }))
    })
    .await??;

    Ok(Json(folder))
}

#[post("folder")]
pub async fn create_folder(
    pool: Data<Pool>,
    request: Sanitized<Json<Vec<CreateFolderRequest>>>,
    device: UserDevice,
    ws_server: Data<notify::ws::AddrServer>,
) -> Result<Json<CreatedFolder>, HttpError> {
    let created_folder = block(move || {
        let device_folders_to_create = request.0 .0;

        let mut conn = pool.get().unwrap();

        let device_ids = user_devices::table
            .filter(user_devices::user_id.eq(device.user_id))
            .filter(user_devices::device_id.ne(device.device_id))
            .select(user_devices::device_id)
            .load::<i32>(&mut conn)?;

        let device_not_exist_in_request = device_folders_to_create
            .iter()
            .find(|device_folder| {
                device_ids
                    .iter()
                    .find(|id| **id == device_folder.device_id)
                    .is_none()
            })
            .is_some();

        if device_ids.len() != device_folders_to_create.iter().map(|d| d.device_id).collect::<HashSet<i32>>().len() || device_not_exist_in_request {
            return Err(HttpError::unprocessable_entity("devices_mismatch"));
        }

        let folder: Folder = diesel::insert_into(folders::table)
            .values(folders::user_id.eq(device.user_id))
            .get_result(&mut conn)?;

        diesel::insert_into(device_folders::table)
            .values(
                device_folders_to_create
                    .into_iter()
                    .map(|folder_to_create| {
                        (
                            device_folders::folder_id.eq(folder.id),
                            device_folders::receiver_device_id.eq(folder_to_create.device_id),
                            device_folders::sender_device_id.eq(device.device_id),
                            device_folders::name.eq(folder_to_create.name),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .execute(&mut conn)?;

        ws_server.do_send(SendExclusiveDeviceMessage {
            user_id: device.user_id,
            excluded_device_id: device.device_id,
            message: DeviceMessage::RefreshFolder(folder.id)
        });

        Ok(CreatedFolder { id: folder.id })
    })
    .await??;

    Ok(Json(created_folder))
}

#[delete("folder/{folder_id}")]
pub async fn delete_folder(
    pool: Data<Pool>,
    folder_id: Path<i32>,
    device: UserDevice,
    ws_server: Data<notify::ws::AddrServer>,
) -> Result<Json<HttpMessage>, HttpError> {
    block(move || -> Result<(), HttpError> {
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
            .execute(&mut conn)?;

        ws_server.do_send(SendExclusiveDeviceMessage {
            user_id: device.user_id,
            excluded_device_id: device.device_id,
            message: DeviceMessage::RefreshFolder(folder_id)
        });

        Ok(())
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}

#[post("/note")]
pub async fn create_note(
    pool: Data<Pool>,
    query: Query<FolderId>,
    request: Sanitized<Json<Vec<CreateNoteRequest>>>,
    device: UserDevice,
    ws_server: Data<notify::ws::AddrServer>,
) -> Result<Json<CreatedNote>, HttpError> {
    let note = block(move || {
        let notes_to_create = request.0 .0;

        let mut conn = pool.get().unwrap();

        let folder_id = folders::table
            .filter(folders::id.eq(query.folder_id))
            .filter(folders::state.eq(State::Clean))
            .filter(folders::user_id.eq(device.user_id))
            .select(folders::id)
            .first::<i32>(&mut conn)?;

        let device_ids = user_devices::table
            .filter(user_devices::user_id.eq(device.user_id))
            .filter(user_devices::device_id.ne(device.device_id))
            .select(user_devices::device_id)
            .load::<i32>(&mut conn)?;

        let device_not_exist_in_request = notes_to_create
            .iter()
            .find(|note_to_create| {
                device_ids
                    .iter()
                    .find(|id| **id == note_to_create.device_id)
                    .is_none()
            })
            .is_some();

        if device_ids.len() != notes_to_create.len() || device_not_exist_in_request {
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
                            device_notes::sender_device_id.eq(device.device_id),
                            device_notes::name.eq(note_to_create.name),
                            device_notes::text.eq(note_to_create.text),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .execute(&mut conn)?;

        ws_server.do_send(SendExclusiveDeviceMessage {
            user_id: device.user_id,
            excluded_device_id: device.device_id,
            message: DeviceMessage::RefreshNote { folder_id: note.folder_id, note_id: note.id, commit: note.commit, deleted: false }
        });

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
    device: UserDevice,
) -> Result<Json<responses::Note>, HttpError> {
    let note = block(move || {
        notes::table
            .filter(notes::id.eq(note_id.into_inner()))
            .filter(folders::user_id.eq(device.user_id))
            .inner_join(folders::table)
            .left_join(
                device_notes::table.on(device_notes::note_id
                    .eq(notes::id)
                    .and(device_notes::receiver_device_id.eq(device.device_id))),
            )
            .select((
                notes::id,
                notes::commit,
                notes::state,
                (
                    device_notes::sender_device_id,
                    device_notes::name,
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
    device: UserDevice,
    ws_server: Data<notify::ws::AddrServer>,
) -> Result<Json<Commit>, HttpError> {
    let commit = block(move || -> Result<Commit, HttpError> {
        let mut conn = pool.get().unwrap();

        // TODO make incrementing commit atomic
        let (note_id, folder_id, commit) = notes::table
            .filter(notes::id.eq(note_id.into_inner()))
            .filter(notes::state.eq(State::Clean))
            .filter(folders::user_id.eq(device.user_id))
            .inner_join(folders::table)
            .select((notes::id, notes::folder_id, notes::commit))
            .first::<(i32, i32, i32)>(&mut conn)?;

        if commit != request.commit {
            return Err(HttpError::conflict("commit_mismatch"));
        }

        let device_notes = request.0 .0.device_notes;

        let device_ids = user_devices::table
            .filter(user_devices::user_id.eq(device.user_id))
            .filter(user_devices::device_id.ne(device.device_id))
            .select(user_devices::device_id)
            .load::<i32>(&mut conn)?;

        let device_not_exist_in_request = device_notes
            .iter()
            .find(|note_to_create| {
                device_ids
                    .iter()
                    .find(|id| **id == note_to_create.device_id)
                    .is_none()
            })
            .is_some();

        if device_ids.len() != device_notes.len() || device_not_exist_in_request {
            return Err(HttpError::unprocessable_entity("devices_mismatch"));
        }

        diesel::update(notes::table)
            .filter(notes::id.eq(note_id))
            .set(notes::commit.eq(commit + 1))
            .execute(&mut conn)?;

        for device_note in device_notes {
            diesel::insert_into(device_notes::table)
                .values((
                    device_notes::note_id.eq(note_id),
                    device_notes::receiver_device_id.eq(device_note.device_id),
                    device_notes::sender_device_id.eq(device.device_id),
                    device_notes::name.eq(&device_note.name),
                    device_notes::text.eq(&device_note.text),
                ))
                .on_conflict((device_notes::note_id, device_notes::receiver_device_id))
                .do_update()
                .set((
                    device_notes::sender_device_id.eq(device.device_id),
                    device_notes::name.eq(&device_note.name),
                    device_notes::text.eq(&device_note.text),
                ))
                .execute(&mut conn)?;

            // Remove this device notes since it updated the note
            diesel::delete(device_notes::table)
                .filter(device_notes::note_id.eq(note_id))
                .filter(device_notes::receiver_device_id.eq(device.device_id))
                .execute(&mut conn)?;
        }

        ws_server.do_send(SendExclusiveDeviceMessage {
            user_id: device.user_id,
            excluded_device_id: device.device_id,
            message: DeviceMessage::RefreshNote { folder_id, note_id, commit: commit + 1, deleted: false }
        });

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
    device: UserDevice,
    ws_server: Data<notify::ws::AddrServer>,
) -> Result<Json<HttpMessage>, HttpError> {
    block(move || -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        let (note_id, folder_id, commit) = notes::table
            .filter(notes::id.eq(note_id.into_inner()))
            .filter(notes::state.eq(State::Clean))
            .filter(folders::user_id.eq(device.user_id))
            .inner_join(folders::table)
            .select((notes::id, folders::id, notes::commit))
            .first::<(i32, i32, i32)>(&mut conn)?;

        diesel::update(notes::table)
            .filter(notes::id.eq(note_id))
            .set(notes::state.eq(State::Deleted))
            .execute(&mut conn)?;

        diesel::delete(device_notes::table)
            .filter(device_notes::note_id.eq(note_id))
            .execute(&mut conn)?;

        ws_server.do_send(SendExclusiveDeviceMessage {
            user_id: device.user_id,
            excluded_device_id: device.device_id,
            message: DeviceMessage::RefreshNote { folder_id, note_id, commit, deleted: true }
        });

        Ok(())
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}

#[get("requests")]
pub async fn fetch_requests(pool: Data<Pool>, device: UserDevice) -> Result<Json<Requests>, HttpError> {
    let requests = block(move || {
        let mut conn = pool.get().unwrap();

        let folders: Vec<FolderRequest> = folder_requests::table
            .inner_join(user_devices::table.on(folder_requests::device_id.eq(user_devices::device_id)))
            .filter(user_devices::user_id.eq(device.user_id))
            .filter(folder_requests::device_id.ne(device.device_id))
            .select((folder_requests::folder_id, folder_requests::device_id))
            .load(&mut conn)?;

        let notes: Vec<NoteRequest> = note_requests::table
            .inner_join(user_devices::table.on(note_requests::device_id.eq(user_devices::device_id)))
            .filter(user_devices::user_id.eq(device.user_id))
            .filter(note_requests::device_id.ne(device.device_id))
            .select((note_requests::note_id, note_requests::device_id))
            .load(&mut conn)?;

        Result::<_, HttpError>::Ok(Requests {
            folder_requests: folders,
            note_requests: notes,
        })
    })
    .await??;

    Ok(Json(requests))
}

pub async fn create_requests(
    pool: Data<Pool>,
    device: UserDevice,
    request: Sanitized<Json<CreateRequests>>,
    ws_server: Data<notify::ws::AddrServer>,
) -> Result<Json<HttpMessage>, HttpError> {
    let request = request.0 .0;

    if request.folder_ids.len() == 0 && request.note_ids.len() == 0 {
        return Err(HttpError::unprocessable_entity("no_request_specified"));
    }

    block(move || -> Result<(), HttpError> {
        let mut conn = pool.get().unwrap();

        let folder_count = folders::table
            .filter(folders::user_id.eq(device.user_id))
            .filter(folders::id.eq_any(request.folder_ids.as_slice()))
            .filter(folders::state.eq(State::Clean))
            .select(diesel::dsl::count(folders::id))
            .get_result::<i64>(&mut conn)?;

        if folder_count != request.folder_ids.len() as i64 {
            return Err(HttpError::unprocessable_entity("unknown_folder"));
        }

        let note_count = notes::table
            .filter(folders::user_id.eq(device.user_id))
            .filter(notes::id.eq_any(request.note_ids.as_slice()))
            .filter(notes::state.eq(State::Clean))
            .inner_join(folders::table)
            .select(diesel::dsl::count(notes::id))
            .get_result::<i64>(&mut conn)?;

        if note_count != request.note_ids.len() as i64 {
            return Err(HttpError::unprocessable_entity("unknown_note"));
        }

        conn.transaction(move |conn| {
            let values = request
                .folder_ids
                .into_iter()
                .map(|id| {
                    (
                        folder_requests::folder_id.eq(id),
                        folder_requests::device_id.eq(device.device_id),
                    )
                })
                .collect::<Vec<_>>();

            diesel::insert_into(folder_requests::table)
                .values(values)
                .on_conflict_do_nothing()
                .execute(conn)?;

            let values = request
                .note_ids
                .into_iter()
                .map(|id| {
                    (
                        note_requests::note_id.eq(id),
                        note_requests::device_id.eq(device.device_id),
                    )
                })
                .collect::<Vec<_>>();

            diesel::insert_into(note_requests::table)
                .values(values)
                .on_conflict_do_nothing()
                .execute(conn)
        })?;

        ws_server.do_send(SendExclusiveDeviceMessage {
            user_id: device.user_id,
            excluded_device_id: device.device_id,
            message: DeviceMessage::RefreshRequests,
        });

        Ok(())
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}

#[post("respond-requests")]
pub async fn respond_requests(
    pool: Data<Pool>,
    device: UserDevice,
    request: Sanitized<Json<RespondRequests>>,
    ws_server: Data<notify::ws::AddrServer>,
) -> Result<Json<HttpMessage>, HttpError> {
    let request = request.0 .0;

    if request.folders.len() == 0 && request.notes.len() == 0 {
        return Err(HttpError::unprocessable_entity("no_request_specified"));
    }

    block(move || {
        let mut conn = pool.get().unwrap();

        let requested_folder_ids: Vec<i32> =
            request.folders.iter().map(|req| req.folder_id).collect();
        let requested_note_ids: Vec<i32> = request.notes.iter().map(|req| req.note_id).collect();

        let folder_count = folders::table
            .filter(folders::user_id.eq(device.user_id))
            .filter(folders::id.eq_any(requested_folder_ids.as_slice()))
            .filter(folder_requests::device_id.eq(request.device_id))
            .inner_join(folder_requests::table)
            .select(diesel::dsl::count(folders::id))
            .get_result::<i64>(&mut conn)?;

        if folder_count != requested_folder_ids.len() as i64 {
            return Err(HttpError::unprocessable_entity("unknown_folder"));
        }

        let note_count = notes::table
            .filter(folders::user_id.eq(device.user_id))
            .filter(notes::id.eq_any(requested_note_ids.as_slice()))
            .inner_join(folders::table)
            .inner_join(
                note_requests::table.on(note_requests::note_id
                    .eq(notes::id)
                    .and(note_requests::device_id.eq(request.device_id))),
            )
            .select(diesel::dsl::count(notes::id))
            .get_result::<i64>(&mut conn)?;

        if note_count != requested_note_ids.len() as i64 {
            return Err(HttpError::unprocessable_entity("unknown_note"));
        }

        let device_id = request.device_id;

        conn.transaction(move |conn| -> Result<(), diesel::result::Error> {
            if requested_folder_ids.len() > 0 {
                let values = request
                    .folders
                    .into_iter()
                    .map(|req| {
                        (
                            device_folders::folder_id.eq(req.folder_id),
                            device_folders::receiver_device_id.eq(request.device_id),
                            device_folders::sender_device_id.eq(device.device_id),
                            device_folders::name.eq(req.name),
                        )
                    })
                    .collect::<Vec<_>>();

                diesel::insert_into(device_folders::table)
                    .values(values)
                    .on_conflict_do_nothing()
                    .execute(conn)?;

                diesel::delete(folder_requests::table)
                    .filter(folder_requests::device_id.eq(request.device_id))
                    .filter(folder_requests::folder_id.eq_any(requested_folder_ids.as_slice()))
                    .execute(conn)?;
            }

            if requested_note_ids.len() > 0 {
                let values = request
                    .notes
                    .into_iter()
                    .map(|req| {
                        (
                            device_notes::note_id.eq(req.note_id),
                            device_notes::receiver_device_id.eq(request.device_id),
                            device_notes::sender_device_id.eq(device.device_id),
                            device_notes::name.eq(req.name),
                            device_notes::text.eq(req.text),
                        )
                    })
                    .collect::<Vec<_>>();

                diesel::insert_into(device_notes::table)
                    .values(values)
                    .on_conflict_do_nothing()
                    .execute(conn)?;

                diesel::delete(note_requests::table)
                    .filter(note_requests::device_id.eq(request.device_id))
                    .filter(note_requests::note_id.eq_any(requested_note_ids.as_slice()))
                    .execute(conn)?;
            }

            Ok(())
        })?;

        ws_server.do_send(SendDeviceMessage { user_id: device.user_id, device_id, message: DeviceMessage::RefreshRemote });

        Ok(())
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{Folder, Note},
        requests::CreateRequests,
    };
    use base::{
        sanitize::Sanitized,
        schema::{folder_requests, folders, notes, users, note_requests},
        HttpError, HttpMessage,
    };
    use test_helpers::db::create_pool;
    use chrono::NaiveDateTime;
    use user::test::db::UserDeviceBuilder;
    use notify::test::ws::create_server as create_notify_server;

    use super::create_requests;

    use actix_web::web::{Data, Json};
    use diesel::{prelude::*, PgConnection};

    fn create_folder(
        conn: &mut PgConnection,
        user_id: Option<i32>,
    ) -> Result<Folder, diesel::result::Error> {
        let user_id = if let Some(user_id) = user_id {
            user_id
        } else {
            diesel::insert_into(users::table)
                .values(users::email.eq("folder@email.com"))
                .get_result::<(i32, String, NaiveDateTime)>(conn)?
                .0
        };

        diesel::insert_into(folders::table)
            .values(folders::user_id.eq(user_id))
            .get_result(conn)
    }

    fn create_note(
        conn: &mut PgConnection,
        folder_id: Option<i32>,
    ) -> Result<Note, diesel::result::Error> {
        let folder_id = if let Some(folder_id) = folder_id {
            folder_id
        } else {
            let user_id = diesel::insert_into(users::table)
                .values(users::email.eq("note@email.com"))
                .get_result::<(i32, String, NaiveDateTime)>(conn)?
                .0;

            diesel::insert_into(folders::table)
                .values(folders::user_id.eq(user_id))
                .get_result::<Folder>(conn)?
                .id
        };

        diesel::insert_into(notes::table)
            .values(notes::folder_id.eq(folder_id))
            .get_result(conn)
    }

    #[actix_web::test]
    async fn it_returns_no_request_specified_error_if_ids_are_empty_when_create_request_is_called()
    {
        let pool = create_pool();

        let device = UserDeviceBuilder::default().build(&mut pool.get().unwrap()).unwrap();
        let request = CreateRequests {
            folder_ids: Vec::new(),
            note_ids: Vec::new(),
        };

        let res = create_requests(Data::new(pool), device, Sanitized(Json(request)), Data::new(create_notify_server())).await;

        assert_eq!(
            HttpError::unprocessable_entity("no_request_specified"),
            res.unwrap_err()
        );
    }

    #[actix_web::test]
    async fn it_returns_unknown_folder_error_if_one_of_the_folder_id_does_not_belong_to_user_when_create_request_is_called(
    ) {
        let pool = create_pool();

        let (device, folder) = {
            let mut conn = pool.get().unwrap();
            let device = UserDeviceBuilder::default().build(&mut conn).unwrap();
            let folder = create_folder(&mut conn, None).unwrap();

            (device, folder)
        };

        let request = CreateRequests {
            folder_ids: vec![folder.id],
            note_ids: Vec::new(),
        };

        let res = create_requests(Data::new(pool), device, Sanitized(Json(request)), Data::new(create_notify_server())).await;

        assert_eq!(
            HttpError::unprocessable_entity("unknown_folder"),
            res.unwrap_err()
        );
    }

    #[actix_web::test]
    async fn it_returns_unknown_note_error_if_one_of_the_note_id_does_not_belong_to_user_when_create_request_is_called(
    ) {
        let pool = create_pool();

        let (device, note) = {
            let mut conn = pool.get().unwrap();
            let device = UserDeviceBuilder::default().build(&mut conn).unwrap();
            let note = create_note(&mut conn, None).unwrap();

            (device, note)
        };

        let request = CreateRequests {
            folder_ids: Vec::new(),
            note_ids: vec![note.id],
        };

        let res = create_requests(Data::new(pool), device, Sanitized(Json(request)), Data::new(create_notify_server())).await;

        assert_eq!(
            HttpError::unprocessable_entity("unknown_note"),
            res.unwrap_err()
        );
    }

    #[actix_web::test]
    async fn it_creates_rows_for_folders_and_notes_when_create_request_is_called() {
        let pool = create_pool();

        let (device, folder, note) = {
            let mut conn = pool.get().unwrap();
            let device = UserDeviceBuilder::default().build(&mut conn).unwrap();
            let folder = create_folder(&mut conn, Some(device.user_id)).unwrap();
            let note = create_note(&mut conn, Some(folder.id)).unwrap();

            (device, folder, note)
        };

        let request = CreateRequests {
            folder_ids: vec![folder.id],
            note_ids: vec![note.id],
        };

        let res = create_requests(
            Data::new(pool.clone()),
            device.clone(),
            Sanitized(Json(request)),
            Data::new(create_notify_server()),
        )
        .await;

        assert_eq!(HttpMessage::success(), res.unwrap().0);

        let folder_request_exist = diesel::select(diesel::dsl::exists(
            folder_requests::table
                .filter(folder_requests::folder_id.eq(folder.id))
                .filter(folder_requests::device_id.eq(device.device_id)),
        ))
        .get_result::<bool>(&mut pool.get().unwrap())
        .unwrap();

        let note_request_exist = diesel::select(diesel::dsl::exists(
            note_requests::table
                .filter(note_requests::note_id.eq(note.id))
                .filter(note_requests::device_id.eq(device.device_id)),
        ))
        .get_result::<bool>(&mut pool.get().unwrap())
        .unwrap();

        assert!(folder_request_exist);
        assert!(note_request_exist);
    }
}
