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
    models::Device,
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

        let (email, fingerprint, created_at) = pending_devices::table
            .filter(pending_devices::fingerprint.eq(&request.fingerprint))
            .filter(users::id.eq(device.user_id))
            .inner_join(users::table.on(pending_devices::email.eq(users::email)))
            .select((
                pending_devices::email,
                pending_devices::fingerprint,
                pending_devices::created_at,
            ))
            .first::<(String, String, NaiveDateTime)>(&mut conn)?;

        let minutes_since_fingerprint_received = Utc::now()
            .naive_utc()
            .signed_duration_since(created_at)
            .num_minutes();

        if minutes_since_fingerprint_received > 5 {
            return Err(HttpError::unprocessable_entity("fingerprint_expired"));
        }

        diesel::delete(pending_devices::table)
            .filter(pending_devices::email.eq(email))
            .filter(pending_devices::fingerprint.eq(fingerprint))
            .execute(&mut conn)?;

        let device: Device = diesel::insert_into(devices::table)
            .values(devices::user_id.eq(device.user_id))
            .get_result(&mut conn)?;

        Result::<i32, HttpError>::Ok(device.id)
    })
    .await??;

    Ok(Json(CreatedDevice { id: device_id }))
}
