use actix_web::{
    get, post, put,
    web::{block, Data, Json, Path}, delete, http::StatusCode,
};
use diesel::prelude::*;

use base::{sanitize::Sanitized, schemas::{folders, notes}, types::Pool, HttpError};
use user::models::User;

use crate::{
    models::{Folder, Note, State},
    requests::{CreateFolderRequest, CreateNoteRequest, UpdateNoteRequest},
    responses::Commit
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

        diesel::update(folders::table)
            .filter(folders::id.eq(folder_id))
            .set((
                folders::state.eq(State::Deleted),
            ))
            .execute(&conn)?;

        // This query can be removed at some point since at database level, deleting a folder will
        // be cascaded to notes
        diesel::delete(notes::table)
            .filter(notes::folder_id.eq(folder_id))
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
) -> Result<Json<Vec<Note>>, HttpError> {
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
            .order(notes::id.desc())
            .select(notes::all_columns)
            .load(&conn)
    })
    .await??
    .into_iter()
    .collect();

    Ok(Json(notes))
}

#[get("folder/{folder_id}/commits")]
pub async fn fetch_commits(
    pool: Data<Pool>,
    folder_id: Path<i32>,
    user: User,
) -> Result<Json<Vec<Commit>>, HttpError> {
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
            .order(notes::id.desc())
            .select((notes::id, notes::commit, notes::state))
            .load::<(i32, i32, State)>(&conn)
    })
    .await??
    .into_iter()
    .map(|commit| Commit {
        note_id: commit.0,
        commit: commit.1,
        state: commit.2,
    })
    .collect();

    Ok(Json(commits))
}

#[get("note/{note_id}")]
pub async fn fetch_note(pool: Data<Pool>, note_id: Path<i32>, user: User) -> Result<Json<Note>, HttpError> {
    let note = block(move || {
        notes::table
            .filter(notes::id.eq(note_id.into_inner()))
            .filter(notes::state.eq(State::Clean.as_str()))
            .filter(folders::user_id.eq(user.id))
            .inner_join(folders::table)
            .select(notes::all_columns)
            .first(&pool.get().unwrap())
    })
    .await??;

    Ok(Json(note))
}

#[post("/note")]
pub async fn create_note(
    pool: Data<Pool>,
    request: Sanitized<Json<CreateNoteRequest>>,
    user: User,
) -> Result<Json<Note>, HttpError> {
    let note = block(move || {
        let conn = pool.get().unwrap();

        let folder_id = folders::table
            .filter(folders::id.eq(request.folder_id))
            .filter(folders::user_id.eq(user.id))
            .filter(folders::state.eq(State::Clean))
            .select(folders::id)
            .first::<i32>(&conn)?;

        diesel::insert_into(notes::table)
            .values((
                notes::folder_id.eq(folder_id),
                notes::title.eq(&request.title),
                notes::text.eq(&request.text)
            ))
            .get_result::<Note>(&conn)
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
) -> Result<Json<Commit>, HttpError> {
    let commit = block(move || -> Result<Commit, HttpError> {
        let conn = pool.get().unwrap();

        // TODO make incrementing commit atomic
        let (note_id, commit) = notes::table
            .filter(notes::id.eq(note_id.into_inner()))
            .filter(notes::state.eq(State::Clean.as_str()))
            .filter(folders::user_id.eq(user.id))
            .inner_join(folders::table)
            .select((notes::id, notes::commit))
            .first::<(i32, i32)>(&conn)?;

        if commit != request.commit {
            return Err(HttpError {
                code: StatusCode::CONFLICT,
                error: "commit_not_matches",
                message: None,
            });
        }

        diesel::update(notes::table)
            .filter(notes::id.eq(note_id))
            .set((notes::commit.eq(commit + 1), notes::title.eq(&request.title), notes::text.eq(&request.text)))
            .execute(&conn)?;

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
            .execute(&conn)
    })
    .await??;

    Ok("")
}
