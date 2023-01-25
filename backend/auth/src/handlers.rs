use base::{
    crypto::Crypto,
    models::Token,
    sanitize::Sanitized,
    schema::{devices, pending_users, users},
    types::Pool,
    HttpError, HttpMessage,
};

use actix_web::{
    post,
    web::{block, Data, Json},
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rand::seq::SliceRandom;

use crate::{
    models::PendingUser,
    requests::{SendCode, SignUp},
    responses::TokenResponse,
};

#[post("sign-up")]
pub async fn sign_up(
    crypto: Data<Crypto>,
    pool: Data<Pool>,
    request: Sanitized<Json<SignUp>>,
) -> Result<Json<TokenResponse>, HttpError> {
    let token = block(move || -> Result<String, HttpError> {
        let mut conn = pool.get().unwrap();

        let pending_user = pending_users::table
            .filter(pending_users::email.eq(&request.email))
            .first::<PendingUser>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => HttpError::not_found("email_not_found"),
                _ => e.into(),
            })?;

        let minutes_since_code_sent = Utc::now()
            .naive_utc()
            .signed_duration_since(pending_user.updated_at)
            .num_minutes();

        if minutes_since_code_sent > 5 {
            return Err(HttpError::unprocessable_entity("code_expired"));
        }

        if pending_user.code != request.code {
            return Err(HttpError::unprocessable_entity("invalid_code"));
        }

        let (user_id, _, _) = diesel::insert_into(users::table)
            .values(users::email.eq(pending_user.email))
            .get_result::<(i32, String, NaiveDateTime)>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => HttpError::conflict("user_already_exists"),
                _ => e.into(),
            })?;

        let (device_id, _) = diesel::insert_into(devices::table)
            .values(devices::user_id.eq(user_id))
            .get_result::<(i32, i32)>(&mut conn)?;

        diesel::delete(pending_users::table)
            .filter(pending_users::email.eq(&request.email))
            .execute(&mut conn)?;

        crypto.encode(&Token::new(device_id)).map_err(|e| e.into())
    })
    .await??;

    Ok(Json(TokenResponse::new(token)))
}

#[post("send-code")]
pub async fn send_code(
    pool: Data<Pool>,
    request: Sanitized<Json<SendCode>>,
) -> Result<Json<HttpMessage>, HttpError> {
    block(move || -> Result<(), HttpError> {
        let mut conn = pool.get().unwrap();

        let user_exists = diesel::dsl::select(diesel::dsl::exists(
            users::table.filter(users::email.eq(&request.email)),
        ))
        .get_result::<bool>(&mut conn)?;

        if user_exists {
            return Err(HttpError::unprocessable_entity("user_exists"));
        }

        let code: String = b"0123456789"
            .choose_multiple(&mut rand::thread_rng(), 8)
            .map(|num| char::from(*num))
            .collect();

        diesel::insert_into(pending_users::table)
            .values((
                pending_users::code.eq(&code),
                pending_users::email.eq(&request.email),
            ))
            .on_conflict(pending_users::email)
            .do_update()
            .set(pending_users::code.eq(&code))
            .execute(&mut conn)?;

        Ok(())
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}
