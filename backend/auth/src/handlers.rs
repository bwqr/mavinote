use base::models::Token;
use base::schemas::users;
use base::{crypto::Crypto, sanitize::Sanitized, types::Pool, HttpError};

use actix_web::HttpResponse;
use actix_web::{
    post, put,
    web::{block, Data, Json},
};
use diesel::prelude::*;

use crate::requests::{Login, SignUp};
use crate::responses::TokenResponse;
use crate::Error;

#[post("login")]
pub async fn login(
    crypto: Data<Crypto>,
    pool: Data<Pool>,
    login_request: Sanitized<Json<Login>>,
) -> Result<Json<TokenResponse>, HttpError> {
    let token = block(move || -> Result<String, HttpError> {
        let password = crypto.sign512(&login_request.password);

        let user_id = users::table
            .filter(
                users::email
                    .eq(&login_request.email)
                    .and(users::password.eq(password)),
            )
            .select(users::id)
            .first::<i32>(&pool.get().unwrap())
            .map_err(|e| -> HttpError {
                match e {
                    diesel::result::Error::NotFound => Error::InvalidCredentials.into(),
                    e => e.into(),
                }
            })?;

        crypto.encode(&Token::new(user_id)).map_err(|e| e.into())
    })
    .await??;

    Ok(Json(TokenResponse::new(token)))
}

#[put("sign-up")]
pub async fn sign_up(
    crypto: Data<Crypto>,
    pool: Data<Pool>,
    sign_up_request: Sanitized<Json<SignUp>>,
) -> Result<HttpResponse, HttpError> {
    block(move || {
        let password = crypto.sign512(&sign_up_request.password);

        diesel::insert_into(users::table)
            .values((
                users::name.eq(&sign_up_request.name),
                users::email.eq(&sign_up_request.email),
                users::password.eq(password),
            ))
            .execute(&pool.get().unwrap())
            .map_err(|e| -> HttpError {
                match e {
                    diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::UniqueViolation,
                        _,
                    ) => Error::UserExists.into(),
                    e => e.into(),
                }
            })
    })
    .await??;

    Ok(HttpResponse::Created().finish())
}
