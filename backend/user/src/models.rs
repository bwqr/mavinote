use base::{
    models::{Token, TokenKind, UNEXPECTED_TOKEN_KIND},
    schema::{devices, user_devices},
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

#[derive(Queryable, Serialize)]
pub struct Device {
    pub id: i32,
    pub pubkey: String,
    pub created_at: NaiveDateTime,
}

pub const DEVICE_COLUMNS: (devices::id, devices::pubkey, devices::created_at) =
    (devices::id, devices::pubkey, devices::created_at);

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Queryable)]
pub struct UserDevice {
    pub user_id: i32,
    pub device_id: i32
}

impl FromRequest for UserDevice {
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
            .map(|token| (token.user_id, token.device_id, token.kind.clone()));

        Box::pin(async move {
            let map_err = |message: &'static str| HttpError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                error: "missing_data",
                message: Some(String::from(message)),
            };

            let mut conn = conn.map_err(map_err)?;
            let (user_id, device_id, kind) = token.map_err(map_err)?;

            if kind != TokenKind::Device {
                return Err(UNEXPECTED_TOKEN_KIND);
            }

            block(move || {
                user_devices::table
                    .filter(user_devices::user_id.eq(user_id))
                    .filter(user_devices::device_id.eq(device_id))
                    .select((user_devices::user_id, user_devices::device_id))
                    .first::<UserDevice>(&mut conn)
                    .map_err(|e| match e {
                        diesel::result::Error::NotFound => HttpError {
                            code: StatusCode::UNAUTHORIZED,
                            error: "device_deleted",
                            message: None,
                        },
                        e => e.into(),
                    })
            })
            .await?
        })
    }
}
