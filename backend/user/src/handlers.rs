use actix_web::{
    get,
    web::{self, Data, Json},
};
use diesel::prelude::*;

use base::{schema::devices, types::Pool, HttpError};

use crate::models::Device;

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
