use actix_web::{
    delete, get, post, put,
    web::{block, Data, Json, Path, Query},
};
use diesel::prelude::*;

use base::{
    sanitize::Sanitized,
    schema::{
        device_folders, device_notes, devices, folder_requests, folders, note_requests, notes,
    },
    types::Pool,
    HttpError, HttpMessage,
};
use user::models::{Device, DEVICE_COLUMNS};

use crate::{
    models::{Folder, Note, State},
    requests::{
        CreateFolderRequest, CreateNoteRequest, CreateRequests, FolderId, RespondRequests,
        UpdateNoteRequest,
    },
    responses::{self, Commit, CreatedFolder, CreatedNote, FolderRequest, NoteRequest, Requests},
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
            .select(DEVICE_COLUMNS)
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

#[post("/note")]
pub async fn create_note(
    pool: Data<Pool>,
    query: Query<FolderId>,
    request: Sanitized<Json<Vec<CreateNoteRequest>>>,
    device: Device,
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

        let devices = devices::table
            .filter(devices::user_id.eq(device.user_id))
            .filter(devices::id.ne(device.id))
            .select(DEVICE_COLUMNS)
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
                            device_notes::name.eq(note_to_create.name),
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
            .select(DEVICE_COLUMNS)
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
                    device_notes::name.eq(device_note.name),
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

#[get("requests")]
pub async fn fetch_requests(pool: Data<Pool>, device: Device) -> Result<Json<Requests>, HttpError> {
    let requests = block(move || {
        let mut conn = pool.get().unwrap();

        let folders: Vec<FolderRequest> = folder_requests::table
            .filter(devices::user_id.eq(device.user_id))
            .filter(devices::id.ne(device.id))
            .inner_join(devices::table)
            .select((folder_requests::folder_id, devices::id))
            .get_results(&mut conn)?;

        let notes: Vec<NoteRequest> = note_requests::table
            .filter(devices::user_id.eq(device.user_id))
            .filter(devices::id.ne(device.id))
            .inner_join(devices::table)
            .select((note_requests::note_id, devices::id))
            .get_results(&mut conn)?;

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
    device: Device,
    request: Sanitized<Json<CreateRequests>>,
) -> Result<Json<HttpMessage>, HttpError> {
    let request = request.0 .0;

    if request.folder_ids.len() == 0 && request.note_ids.len() == 0 {
        return Err(HttpError::unprocessable_entity("no_request_specified"));
    }

    block(move || {
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
                        folder_requests::device_id.eq(device.id),
                    )
                })
                .collect::<Vec<_>>();

            diesel::insert_into(folder_requests::table)
                .values(values)
                .execute(conn)
                .map_err(|e| match e {
                    diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::UniqueViolation,
                        _,
                    ) => HttpError::conflict("one_request_already_exists"),
                    _ => e.into(),
                })?;

            let values = request
                .note_ids
                .into_iter()
                .map(|id| {
                    (
                        note_requests::note_id.eq(id),
                        note_requests::device_id.eq(device.id),
                    )
                })
                .collect::<Vec<_>>();

            diesel::insert_into(note_requests::table)
                .values(values)
                .execute(conn)
                .map_err(|e| match e {
                    diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::UniqueViolation,
                        _,
                    ) => HttpError::conflict("one_request_already_exists"),
                    _ => e.into(),
                })
        })
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}

#[post("respond-requests")]
pub async fn respond_requests(
    pool: Data<Pool>,
    device: Device,
    request: Sanitized<Json<RespondRequests>>,
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

        conn.transaction(move |conn| {
            if requested_folder_ids.len() > 0 {
                let values = request
                    .folders
                    .into_iter()
                    .map(|req| {
                        (
                            device_folders::folder_id.eq(req.folder_id),
                            device_folders::receiver_device_id.eq(request.device_id),
                            device_folders::sender_device_id.eq(device.id),
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
                            device_notes::sender_device_id.eq(device.id),
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
        })
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
        schema::{devices, folder_requests, folders, notes, users, note_requests},
        types::Pool,
        HttpError, HttpMessage,
    };
    use chrono::NaiveDateTime;
    use user::models::Device;

    use super::create_requests;

    use actix_web::web::{Data, Json};
    use diesel::{prelude::*, r2d2::ConnectionManager, PgConnection};

    fn create_pool() -> Pool {
        let conn_info = "postgres://mavinote:toor@127.0.0.1/mavinote_test";
        let manager = ConnectionManager::<PgConnection>::new(conn_info);

        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        pool.get().unwrap().begin_test_transaction().unwrap();

        pool
    }

    fn create_device(conn: &mut PgConnection) -> Result<Device, diesel::result::Error> {
        let (user_id, _, _): (i32, String, NaiveDateTime) = diesel::insert_into(users::table)
            .values(users::email.eq("device@email.com"))
            .get_result(conn)?;

        diesel::insert_into(devices::table)
            .values((devices::user_id.eq(user_id), devices::pubkey.eq("pubkey"), devices::password.eq("password")))
            .get_result::<(i32, i32, String, String)>(conn)
            .map(|row| Device { id: row.0, user_id: row.1, pubkey: row.2 })
    }

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

        let device = create_device(&mut pool.get().unwrap()).unwrap();
        let request = CreateRequests {
            folder_ids: Vec::new(),
            note_ids: Vec::new(),
        };

        let res = create_requests(Data::new(pool), device, Sanitized(Json(request))).await;

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
            let device = create_device(&mut conn).unwrap();
            let folder = create_folder(&mut conn, None).unwrap();

            (device, folder)
        };

        let request = CreateRequests {
            folder_ids: vec![folder.id],
            note_ids: Vec::new(),
        };

        let res = create_requests(Data::new(pool), device, Sanitized(Json(request))).await;

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
            let device = create_device(&mut conn).unwrap();
            let note = create_note(&mut conn, None).unwrap();

            (device, note)
        };

        let request = CreateRequests {
            folder_ids: Vec::new(),
            note_ids: vec![note.id],
        };

        let res = create_requests(Data::new(pool), device, Sanitized(Json(request))).await;

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
            let device = create_device(&mut conn).unwrap();
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
        )
        .await;

        assert_eq!(HttpMessage::success(), res.unwrap().0);

        let folder_request_exist = diesel::select(diesel::dsl::exists(
            folder_requests::table
                .filter(folder_requests::folder_id.eq(folder.id))
                .filter(folder_requests::device_id.eq(device.id)),
        ))
        .get_result::<bool>(&mut pool.get().unwrap())
        .unwrap();

        let note_request_exist = diesel::select(diesel::dsl::exists(
            note_requests::table
                .filter(note_requests::note_id.eq(note.id))
                .filter(note_requests::device_id.eq(device.id)),
        ))
        .get_result::<bool>(&mut pool.get().unwrap())
        .unwrap();

        assert!(folder_request_exist);
        assert!(note_request_exist);
    }
}
