use actix_web::{
    get, post, put,
    web::{block, Data, Json, Path}, http::StatusCode, delete,
};
use diesel::prelude::*;

use base::{sanitize::Sanitized, schemas::{folders, notes, commits}, types::Pool, HttpError};
use user::models::User;

use crate::{
    models::{Folder, Note, Commit, State},
    requests::{CreateFolderRequest, CreateNoteRequest, UpdateNoteRequest},
    responses::{Commit as CommitResponse, Note as NoteResponse}
};

#[get("folders")]
pub async fn fetch_folders(pool: Data<Pool>, user: User) -> Result<Json<Vec<Folder>>, HttpError> {
    let folders = block(move ||
        folders::table.filter(folders::user_id.eq(user.id))
            .load(&pool.get().unwrap())
    ).await??;

    Ok(Json(folders))
}

#[post("folder")]
pub async fn create_folder(
    pool: Data<Pool>,
    request: Sanitized<Json<CreateFolderRequest>>,
    user: User,
) -> Result<Json<Folder>, HttpError> {
    let folder = block(move || {
        diesel::insert_into(folders::table)
            .values((
                folders::user_id.eq(user.id),
                folders::name.eq(&request.name),
            ))
            .get_result(&pool.get().unwrap())
    })
    .await??;

    Ok(Json(folder))
}

#[delete("folder/{folder_id}")]
pub async fn delete_folder(
    pool: Data<Pool>,
    folder_id: Path<i32>,
    user: User,
) -> Result<&'static str, HttpError> {
    block(move || {
        let conn = pool.get().unwrap(); 

        let folder_id = folders::table
            .filter(folders::id.eq(folder_id.into_inner()))
            .filter(folders::user_id.eq(user.id))
            .filter(folders::state.eq(State::Clean))
            .select(folders::id)
            .first::<i32>(&conn)?;

        let note_ids = notes::table
            .filter(notes::folder_id.eq(folder_id))
            .select(notes::id)
            .load::<i32>(&conn)?;

        diesel::update(folders::table)
            .filter(folders::id.eq(folder_id))
            .set((
                folders::state.eq(State::Deleted),
                //folders::name.eq(""),
            ))
            .execute(&conn)?;

        diesel::delete(notes::table)
            .filter(notes::folder_id.eq(folder_id))
            .execute(&conn)?;

        diesel::delete(commits::table)
            .filter(commits::note_id.eq_any(note_ids))
            .execute(&conn)
    })
        .await??;

    Ok("")
}

#[get("folder/{folder_id}/notes")]
pub async fn fetch_notes(
    pool: Data<Pool>,
    folder_id: Path<i32>,
    user: User,
) -> Result<Json<Vec<NoteResponse>>, HttpError> {
    let notes = block(move || {
        let conn = pool.get().unwrap();

        let folder_id = folders::table
            .filter(folders::id.eq(folder_id.into_inner()))
            .filter(folders::user_id.eq(user.id))
            .filter(folders::state.eq(State::Clean))
            .select(folders::id)
            .first::<i32>(&conn)?;

        notes::table
            .filter(notes::folder_id.eq(folder_id))
            .left_join(commits::table)
            .order((notes::id.desc(), commits::id.desc()))
            .select((notes::all_columns, commits::all_columns.nullable()))
            .distinct_on(notes::id)
            .load(&conn)
    })
    .await??
    .into_iter()
    .filter(|note: &(Note, Option<Commit>)| note.1.is_some())
    .map(|note| NoteResponse::from((note.0, note.1.unwrap())))
    .collect();

    Ok(Json(notes))
}

#[get("folder/{folder_id}/commits")]
pub async fn fetch_commits(
    pool: Data<Pool>,
    folder_id: Path<i32>,
    user: User,
) -> Result<Json<Vec<CommitResponse>>, HttpError> {
    let commits = block(move || {
        let conn = pool.get().unwrap();

        let folder_id = folders::table
            .filter(folders::id.eq(folder_id.into_inner()))
            .filter(folders::user_id.eq(user.id))
            .filter(folders::state.eq(State::Clean))
            .select(folders::id)
            .first::<i32>(&conn)?;

        notes::table
            .filter(notes::folder_id.eq(folder_id))
            .left_join(commits::table)
            .order((notes::id.desc(), commits::id.desc()))
            .select((notes::id, notes::state, commits::id.nullable()))
            .distinct_on(notes::id)
            .load(&conn)
    })
    .await??
    .into_iter()
    .filter(|commit: &(i32, State, Option<i32>)| commit.2.is_some())
    .map(|commit| CommitResponse {
        note_id: commit.0,
        state: commit.1,
        commit_id: commit.2.unwrap(),
    })
    .collect();

    Ok(Json(commits))
}

#[get("note/{note_id}")]
pub async fn fetch_note(pool: Data<Pool>, note_id: Path<i32>, user: User) -> Result<Json<NoteResponse>, HttpError> {
    let note_and_commit = block(move || {
        notes::table
            .filter(notes::id.eq(note_id.into_inner()))
            .filter(notes::state.eq(State::Clean.as_str()))
            .filter(folders::user_id.eq(user.id))
            .inner_join(folders::table)
            .left_join(commits::table)
            .order(commits::id.desc())
            .select((notes::all_columns, commits::all_columns.nullable()))
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
    user: User,
) -> Result<Json<NoteResponse>, HttpError> {
    let note = block(move || -> Result<NoteResponse, HttpError> {
        let conn = pool.get().unwrap();

        let folder_id = folders::table
            .filter(folders::id.eq(request.folder_id))
            .filter(folders::user_id.eq(user.id))
            .filter(folders::state.eq(State::Clean))
            .select(folders::id)
            .first::<i32>(&conn)?;

        let note = diesel::insert_into(notes::table)
            .values((
                notes::folder_id.eq(folder_id),
                notes::title.eq(&request.title),
            ))
            .get_result::<Note>(&conn)?;

        let note_commit = diesel::insert_into(commits::table)
            .values((
                commits::note_id.eq(note.id),
                commits::text.eq(&request.text),
            ))
            .get_result(&conn)?;

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
    user: User,
) -> Result<Json<CommitResponse>, HttpError> {
    let commit = block(move || -> Result<CommitResponse, HttpError> {
        let conn = pool.get().unwrap();

        let note_id = notes::table
            .filter(notes::id.eq(note_id.into_inner()))
            .filter(notes::state.eq(State::Clean.as_str()))
            .filter(folders::user_id.eq(user.id))
            .inner_join(folders::table)
            .select(notes::id)
            .first::<i32>(&conn)?;

        diesel::update(notes::table)
            .filter(notes::id.eq(note_id))
            .set(notes::title.eq(&request.title))
            .execute(&conn)?;

        diesel::insert_into(commits::table)
            .values((
                commits::note_id.eq(note_id),
                commits::text.eq(&request.text),
            ))
            .get_result::<Commit>(&conn)
            .map(|commit| CommitResponse {
                commit_id: commit.id,
                state: State::Clean,
                note_id,
            })
            .map_err(|e| e.into())
    })
    .await??;

    Ok(Json(commit))
}

#[delete("note/{note_id}")]
pub async fn delete_note(
    pool: Data<Pool>,
    note_id: Path<i32>,
    user: User,
) -> Result<&'static str, HttpError> {
    block(move || {
        let conn = pool.get().unwrap();

        let note_id = notes::table
            .filter(notes::id.eq(note_id.into_inner()))
            .filter(folders::user_id.eq(user.id))
            .inner_join(folders::table)
            .select(notes::id)
            .first::<i32>(&conn)?;

        diesel::update(notes::table)
            .filter(notes::id.eq(note_id))
            .set((
                notes::state.eq(State::Deleted),
                //notes::title.eq(None as Option<State>),
            ))
            .execute(&conn)?;

        diesel::delete(commits::table)
            .filter(commits::note_id.eq(note_id))
            .execute(&conn)
    })
    .await??;

    Ok("")
}
