use base::{HttpError, types::Pool, models::Token, schemas::users};

use actix_web::{FromRequest, web::{Data, block}, http::StatusCode, HttpMessage};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Queryable;
use futures::future::LocalBoxFuture;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

pub const USER_COLUMNS: (users::id, users::name, users::email, users::created_at) = (
    users::id,
    users::name,
    users::email,
    users::created_at,
);

impl FromRequest for User {
    type Error = HttpError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let conn = req.app_data::<Data<Pool>>()
            .ok_or("Pool could not extracted from request in impl FromRequest for User")
            .map(|pool| pool.get().unwrap());

        let user_id = req.extensions().get::<Token>()
            .ok_or("Token could not extracted from request in impl FromRequest for User")
            .map(|token| token.user_id);

        Box::pin(async move {
            let map_err = |message: &'static str| HttpError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                error: "missingData",
                message: Some(String::from(message)),
            };

            let conn = conn.map_err(map_err)?;
            let user_id = user_id.map_err(map_err)?;

            block(move || {
                users::table
                    .find(user_id)
                    .select(USER_COLUMNS)
                    .first::<User>(&conn)
                    .map_err(|e| e.into())
            })
                .await?
        })
    }
}
