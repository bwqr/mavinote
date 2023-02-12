use actix_web::{
    get, post,
    web::{self, block, Data, Json},
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use base::{
    sanitize::Sanitized,
    schema::{devices, pending_devices, users},
    types::Pool,
    HttpError
};

use crate::{
    models::{Device, DEVICE_COLUMNS},
    requests::AddDevice,
    responses::CreatedDevice,
};

#[get("/devices")]
pub async fn fetch_devices(
    pool: Data<Pool>,
    device: Device,
) -> Result<Json<Vec<Device>>, HttpError> {
    let devices = web::block(move || {
        devices::table
            .filter(devices::user_id.eq(device.user_id))
            .filter(devices::id.ne(device.id))
            .select(DEVICE_COLUMNS)
            .load(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(devices))
}

#[post("/device")]
pub async fn add_device(
    pool: Data<Pool>,
    device: Device,
    request: Sanitized<Json<AddDevice>>,
) -> Result<Json<CreatedDevice>, HttpError> {
    let device_id = block(move || {
        let mut conn = pool.get().unwrap();

        let (email, pubkey, created_at) = pending_devices::table
            .filter(pending_devices::pubkey.eq(&request.pubkey))
            .filter(users::id.eq(device.user_id))
            .inner_join(users::table.on(pending_devices::email.eq(users::email)))
            .select((
                pending_devices::email,
                pending_devices::pubkey,
                pending_devices::created_at,
            ))
            .first::<(String, String, NaiveDateTime)>(&mut conn)?;

        let minutes_since_pubkey_received = Utc::now()
            .naive_utc()
            .signed_duration_since(created_at)
            .num_minutes();

        if minutes_since_pubkey_received > 5 {
            return Err(HttpError::unprocessable_entity("pubkey_expired"));
        }

        diesel::delete(pending_devices::table)
            .filter(pending_devices::email.eq(email))
            .filter(pending_devices::pubkey.eq(pubkey))
            .execute(&mut conn)?;

        let device_id = diesel::insert_into(devices::table)
            .values(devices::user_id.eq(device.user_id))
            .get_result::<(i32, i32, String, String)>(&mut conn)?
            .0;

        Result::<i32, HttpError>::Ok(device_id)
    })
    .await??;

    Ok(Json(CreatedDevice { id: device_id }))
}
