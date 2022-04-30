use actix_web::{
    get, post, put,
    web::{block, Data, Json, Path},
};
use diesel::prelude::*;

use base::{sanitize::Sanitized, schemas, types::Pool, HttpError};

use crate::{
    models::{Folder, Note},
    requests::{CreateFolderRequest, CreateNoteRequest, UpdateNoteRequest},
};

#[get("folders")]
pub async fn fetch_folders(pool: Data<Pool>) -> Result<Json<Vec<Folder>>, HttpError> {
    let folders = block(move || schemas::folders::table.load(&pool.get().unwrap())).await??;

    Ok(Json(folders))
}

#[post("folder")]
pub async fn create_folder(
    pool: Data<Pool>,
    request: Sanitized<Json<CreateFolderRequest>>,
) -> Result<Json<Folder>, HttpError> {
    let folder = block(move || {
        diesel::insert_into(schemas::folders::table)
            .values((
                schemas::folders::user_id.eq(1),
                schemas::folders::name.eq(&request.name),
            ))
            .get_result(&pool.get().unwrap())
    })
    .await??;

    Ok(Json(folder))
}

#[get("folder/{folder_id}/notes")]
pub async fn fetch_notes(
    pool: Data<Pool>,
    folder_id: Path<i32>,
) -> Result<Json<Vec<Note>>, HttpError> {
    let notes = block(move || {
        schemas::notes::table
            .filter(schemas::notes::folder_id.eq(folder_id.into_inner()))
            .load(&pool.get().unwrap())
    })
    .await??;

    Ok(Json(notes))
}

#[get("note/{note_id}")]
pub async fn fetch_note(pool: Data<Pool>, note_id: Path<i32>) -> Result<Json<Note>, HttpError> {
    let note = block(move || {
        schemas::notes::table
            .filter(schemas::notes::id.eq(note_id.into_inner()))
            .first(&pool.get().unwrap())
    })
    .await??;

    Ok(Json(note))
}

#[post("/note")]
pub async fn create_note(
    pool: Data<Pool>,
    request: Sanitized<Json<CreateNoteRequest>>,
) -> Result<Json<Note>, HttpError> {
    let note = block(move || {
        diesel::insert_into(schemas::notes::table)
            .values((
                schemas::notes::folder_id.eq(request.folder_id),
                schemas::notes::title.eq(&request.title),
                schemas::notes::text.eq(&request.text),
            ))
            .get_result(&pool.get().unwrap())
    })
    .await??;

    Ok(Json(note))
}

#[put("note/{note_id}")]
pub async fn update_note(
    pool: Data<Pool>,
    note_id: Path<i32>,
    request: Sanitized<Json<UpdateNoteRequest>>,
) -> Result<Json<Note>, HttpError> {
    let note = block(move || {
        diesel::update(schemas::notes::table)
            .filter(schemas::notes::id.eq(note_id.into_inner()))
            .set((
                schemas::notes::title.eq(&request.title),
                schemas::notes::text.eq(&request.text),
            ))
            .get_result(&pool.get().unwrap())
    })
    .await??;

    Ok(Json(note))
}
