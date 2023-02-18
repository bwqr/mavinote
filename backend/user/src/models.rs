use base::{
    models::{Token, TokenKind, UNEXPECTED_TOKEN_KIND},
    schema::devices,
    types::Pool,
    HttpError,
};

use actix_web::{
    http::StatusCode,
    web::{block, Data},
    FromRequest, HttpMessage,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Queryable;
use futures::future::LocalBoxFuture;
use serde::Serialize;

#[derive(Clone, Queryable, Serialize)]
pub struct Device {
    pub id: i32,
    pub user_id: i32,
    pub pubkey: String,
}

pub const DEVICE_COLUMNS: (devices::id, devices::user_id, devices::pubkey) =
    (devices::id, devices::user_id, devices::pubkey);

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
}

impl FromRequest for Device {
    type Error = HttpError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let conn = req
            .app_data::<Data<Pool>>()
            .ok_or("Pool could not be extracted from request in impl FromRequest for Device")
            .map(|pool| pool.get().unwrap());

        let token = req
            .extensions()
            .get::<Token>()
            .ok_or("Token could not be extracted from request in impl FromRequest for Device")
            .map(|token| (token.id, token.kind.clone()));

        Box::pin(async move {
            let map_err = |message: &'static str| HttpError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                error: "missing_data",
                message: Some(String::from(message)),
            };

            let mut conn = conn.map_err(map_err)?;
            let (device_id, kind) = token.map_err(map_err)?;

            if kind != TokenKind::Device {
                return Err(UNEXPECTED_TOKEN_KIND);
            }

            block(move || {
                devices::table
                    .find(device_id)
                    .select(DEVICE_COLUMNS)
                    .first::<Device>(&mut conn)
                    .map_err(|e| e.into())
            })
            .await?
        })
    }
}
