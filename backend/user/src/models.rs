use base::{models::Token, schema::devices, types::Pool, HttpError};

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
}

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
            .ok_or("Pool could not extracted from request in impl FromRequest for User")
            .map(|pool| pool.get().unwrap());

        let device_id = req
            .extensions()
            .get::<Token>()
            .ok_or("Token could not extracted from request in impl FromRequest for User")
            .map(|token| token.device_id.clone());

        Box::pin(async move {
            let map_err = |message: &'static str| HttpError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                error: "missing_data",
                message: Some(String::from(message)),
            };

            let mut conn = conn.map_err(map_err)?;
            let device_id = device_id.map_err(map_err)?;

            block(move || {
                devices::table
                    .find(device_id)
                    .first::<Device>(&mut conn)
                    .map_err(|e| e.into())
            })
            .await?
        })
    }
}
