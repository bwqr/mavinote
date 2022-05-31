use actix_web::{
    get, post, put,
    web::{block, Data, Json, Path}, http::StatusCode,
};
use diesel::prelude::*;

use base::{sanitize::Sanitized, schemas, types::Pool, HttpError};

use crate::{
    models::{Folder, Note, Commit},
    requests::{CreateFolderRequest, CreateNoteRequest, UpdateNoteRequest},
    responses::{Commit as CommitResponse, Note as NoteResponse}
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
) -> Result<Json<Vec<CommitResponse>>, HttpError> {
    let commits = block(move || {
        schemas::notes::table
            .filter(schemas::notes::folder_id.eq(folder_id.into_inner()))
            .left_join(schemas::commits::table)
            .order((schemas::notes::id.desc(), schemas::commits::id.desc()))
            .select((schemas::notes::id, schemas::commits::id.nullable()))
            .distinct_on(schemas::notes::id)
            .load(&pool.get().unwrap())
    })
    .await??
    .into_iter()
    .filter(|commit: &(i32, Option<i32>)| commit.1.is_some())
    .map(|commit| CommitResponse {
        note_id: commit.0,
        commit_id: commit.1.unwrap().to_string(),
    })
    .collect();

    Ok(Json(commits))
}

#[get("note/{note_id}")]
pub async fn fetch_note(pool: Data<Pool>, note_id: Path<i32>) -> Result<Json<NoteResponse>, HttpError> {
    let note_and_commit = block(move || {
        schemas::notes::table
            .filter(schemas::notes::id.eq(note_id.into_inner()))
            .left_join(schemas::commits::table)
            .order(schemas::commits::id.desc())
            .select((schemas::notes::all_columns, schemas::commits::all_columns.nullable()))
            .first::<(Note, Option<Commit>)>(&pool.get().unwrap())
    })
    .await??;

    if let (note, Some(commit)) = note_and_commit {
        Ok(Json((note, commit).into()))
    } else {
        log::error!("note {} does not have a commit", note_and_commit.0.id);

        Err(HttpError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            error: "invalidState",
            message: Some("note does not have a commit".to_string()),
        })
    }

}

#[post("/note")]
pub async fn create_note(
    pool: Data<Pool>,
    request: Sanitized<Json<CreateNoteRequest>>,
) -> Result<Json<NoteResponse>, HttpError> {
    let note = block(move || -> Result<NoteResponse, HttpError> {
        let note = diesel::insert_into(schemas::notes::table)
            .values((
                schemas::notes::folder_id.eq(request.folder_id),
                schemas::notes::title.eq(&request.title),
            ))
            .get_result::<Note>(&pool.get().unwrap())?;

        let note_commit = diesel::insert_into(schemas::commits::table)
            .values((
                schemas::commits::note_id.eq(note.id),
                schemas::commits::text.eq(&request.text),
            ))
            .get_result(&pool.get().unwrap())?;

        Ok(NoteResponse::from((note, note_commit)))
    })
    .await??;

    Ok(Json(note))
}

#[put("note/{note_id}")]
pub async fn update_note(
    pool: Data<Pool>,
    note_id: Path<i32>,
    request: Sanitized<Json<UpdateNoteRequest>>,
) -> Result<Json<CommitResponse>, HttpError> {
    let commit = block(move || -> Result<CommitResponse, HttpError> {
        let note_id = note_id.into_inner();

        diesel::update(schemas::notes::table)
            .filter(schemas::notes::id.eq(note_id))
            .set((
                schemas::notes::title.eq(&request.title),
            ))
            .execute(&pool.get().unwrap())?;

        diesel::insert_into(schemas::commits::table)
            .values((
                schemas::commits::note_id.eq(note_id),
                schemas::commits::text.eq(&request.text),
            ))
            .get_result::<Commit>(&pool.get().unwrap())
            .map(|commit| CommitResponse {
                commit_id: commit.id.to_string(),
                note_id,
            })
            .map_err(|e| e.into())
    })
    .await??;

    Ok(Json(commit))
}
