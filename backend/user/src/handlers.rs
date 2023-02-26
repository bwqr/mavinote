use actix_web::web::{self, block, Data, Json};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use base::{
    sanitize::Sanitized,
    schema::{devices, pending_devices, users},
    types::Pool,
    HttpError, HttpMessage
};

use crate::{
    models::{Device, DEVICE_COLUMNS},
    requests::AddDevice,
};

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

pub async fn add_device(
    pool: Data<Pool>,
    ws_server: Data<notify::ws::AddrServer>,
    device: Device,
    request: Sanitized<Json<AddDevice>>,
) -> Result<Json<Device>, HttpError> {
    let (created_device, pending_device_id) = block(move || {
        let mut conn = pool.get().unwrap();

        let device_exists = diesel::dsl::select(diesel::dsl::exists(
            devices::table
                .filter(devices::pubkey.eq(&request.pubkey))
                .filter(devices::user_id.eq(&device.user_id))
        ))
            .get_result::<bool>(&mut conn)?;

        if device_exists {
            return Err(HttpError::conflict("device_already_exists"));
        }

        let (pending_device_id, email, pubkey, password, updated_at) = pending_devices::table
            .filter(pending_devices::pubkey.eq(&request.pubkey))
            .filter(users::id.eq(device.user_id))
            .inner_join(users::table.on(pending_devices::email.eq(users::email)))
            .select(pending_devices::all_columns)
            .first::<(i32, String, String, String, NaiveDateTime)>(&mut conn)?;

        let minutes_since_pubkey_received = Utc::now()
            .naive_utc()
            .signed_duration_since(updated_at)
            .num_minutes();

        if minutes_since_pubkey_received > 5 {
            return Err(HttpError::unprocessable_entity("pubkey_expired"));
        }

        diesel::delete(pending_devices::table)
            .filter(pending_devices::email.eq(&email))
            .filter(pending_devices::pubkey.eq(&pubkey))
            .execute(&mut conn)?;

        let device = diesel::insert_into(devices::table)
            .values((devices::user_id.eq(device.user_id), devices::pubkey.eq(pubkey), devices::password.eq(password)))
            .get_result::<(i32, i32, String, String, NaiveDateTime)>(&mut conn)
            .map(|row| Device { id: row.0, user_id: row.1, pubkey: row.2, created_at: row.4 })?;

        Ok((device, pending_device_id))
    })
    .await??;

    ws_server.do_send(notify::ws::messages::AcceptPendingDevice(pending_device_id));

    Ok(Json(created_device))
}

pub async fn delete_device(
    pool: Data<Pool>,
    device: Device,
) -> Result<Json<HttpMessage>, HttpError> {
    block(move || {
        let mut conn = pool.get().unwrap();

        let device_ids = devices::table
            .filter(devices::user_id.eq(device.user_id))
            .select(devices::id)
            .load::<i32>(&mut conn)?;

        if device_ids.len() == 1 && device_ids[0] == device.id {
            return Err(HttpError::conflict("cannot_delete_only_remaining_device"));
        }

        diesel::delete(devices::table)
            .filter(devices::id.eq(device.id))
            .execute(&mut conn)?;

        Result::<(), HttpError>::Ok(())
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}
