use askama::Template;
use actix_web::{web::{self, block, Data, Json, Payload}, HttpRequest, HttpResponse, http::StatusCode};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use notify::ws::messages::{DeviceMessage, SendDeviceMessage};
use rand::seq::SliceRandom;

use base::{
    sanitize::Sanitized,
    schema::{devices, pending_devices, users, pending_delete_users, user_devices, device_notes, device_folders, note_requests, folder_requests},
    types::Pool,
    HttpError, HttpMessage
};
use notify::mail::{MailRecipient, messages::SendMail};

use crate::{
    models::{Device, DEVICE_COLUMNS, UserDevice},
    requests::{AddDevice, CloseAccount, DeleteDevice},
    templates::CloseAccount as CloseAccountTemplate,
};

pub async fn fetch_devices(
    pool: Data<Pool>,
    device: UserDevice,
) -> Result<Json<Vec<Device>>, HttpError> {
    let devices = web::block(move || {
        user_devices::table
            .inner_join(devices::table)
            .filter(user_devices::user_id.eq(device.user_id))
            .filter(user_devices::device_id.ne(device.device_id))
            .select(DEVICE_COLUMNS)
            .load(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(devices))
}

pub async fn add_device(
    pool: Data<Pool>,
    ws_server: Data<notify::ws::AddrServer>,
    device: UserDevice,
    request: Sanitized<Json<AddDevice>>,
) -> Result<Json<Device>, HttpError> {
    let (created_device, pending_device_id) = block(move || {
        let mut conn = pool.get().unwrap();

        let user_device_exists = diesel::dsl::select(diesel::dsl::exists(
            user_devices::table
                .inner_join(devices::table)
                .filter(devices::pubkey.eq(&request.pubkey))
                .filter(user_devices::user_id.eq(&device.user_id))
        ))
            .get_result::<bool>(&mut conn)?;

        if user_device_exists {
            return Err(HttpError::conflict("device_already_exists"));
        }

        let (pending_device_id, updated_at) = pending_devices::table
            .inner_join(devices::table)
            .filter(devices::pubkey.eq(&request.pubkey))
            .filter(pending_devices::user_id.eq(device.user_id))
            .select((pending_devices::device_id, pending_devices::updated_at))
            .first::<(i32, NaiveDateTime)>(&mut conn)?;

        let minutes_since_pubkey_received = Utc::now()
            .naive_utc()
            .signed_duration_since(updated_at)
            .num_minutes();

        if minutes_since_pubkey_received > 5 {
            return Err(HttpError::unprocessable_entity("expired_pubkey"));
        }

        diesel::delete(pending_devices::table)
            .filter(pending_devices::user_id.eq(&device.user_id))
            .filter(pending_devices::device_id.eq(&pending_device_id))
            .execute(&mut conn)?;

        diesel::insert_into(user_devices::table)
            .values((user_devices::user_id.eq(device.user_id), user_devices::device_id.eq(pending_device_id)))
            .execute(&mut conn)?;

        let device = devices::table
            .find(pending_device_id)
            .select(DEVICE_COLUMNS)
            .first::<Device>(&mut conn)?;

        Ok((device, pending_device_id))
    })
    .await??;

    ws_server.do_send(SendDeviceMessage {
        user_id: device.user_id,
        device_id: pending_device_id,
        message: DeviceMessage::AcceptPendingDevice,
    });

    Ok(Json(created_device))
}

pub async fn delete_device(
    pool: Data<Pool>,
    device: UserDevice,
    device_to_delete: web::Query<DeleteDevice>,
) -> Result<Json<HttpMessage>, HttpError> {
    block(move || {
        let mut conn = pool.get().unwrap();
        let device_to_delete = device_to_delete
            .into_inner()
            .id
            .unwrap_or(device.device_id);

        let device_ids = user_devices::table
            .filter(user_devices::user_id.eq(device.user_id))
            .select(user_devices::device_id)
            .load::<i32>(&mut conn)?;

        if device_ids.iter().find(|d| **d == device_to_delete).is_none() {
            return Err(HttpError::not_found("unknown_device"));
        }

        if device_ids.len() == 1 {
            return Err(HttpError::conflict("cannot_delete_only_remaining_device"));
        }

        diesel::delete(user_devices::table)
            .filter(user_devices::device_id.eq(device_to_delete))
            .execute(&mut conn)?;

        diesel::delete(device_notes::table)
            .filter(
                device_notes::sender_device_id.eq(device_to_delete)
                    .or(device_notes::receiver_device_id.eq(device_to_delete))
            )
            .execute(&mut conn)?;

        diesel::delete(device_folders::table)
            .filter(
                device_folders::sender_device_id.eq(device_to_delete)
                    .or(device_folders::receiver_device_id.eq(device_to_delete))
            )
            .execute(&mut conn)?;

        // Removing requests with respect to device id will result in other requests that belongs
        // to another user to be deleted as well
        diesel::delete(note_requests::table)
            .filter(note_requests::device_id.eq(device_to_delete))
            .execute(&mut conn)?;

        diesel::delete(folder_requests::table)
            .filter(folder_requests::device_id.eq(device_to_delete))
            .execute(&mut conn)?;

        Result::<(), HttpError>::Ok(())
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}

pub async fn send_close_account_code(
    pool: Data<Pool>,
    device: UserDevice,
    mail_recipient: Data<MailRecipient>,
) -> Result<Json<HttpMessage>, HttpError> {
    block(move || {
        let mut conn = pool.get().unwrap();

        let code: String = b"0123456789"
            .choose_multiple(&mut rand::thread_rng(), 8)
            .map(|num| char::from(*num))
            .collect();

        let email = users::table
            .filter(users::id.eq(device.user_id))
            .select(users::email)
            .first::<String>(&mut conn)?;

        diesel::insert_into(pending_delete_users::table)
            .values((pending_delete_users::user_id.eq(device.user_id), pending_delete_users::code.eq(&code)))
            .on_conflict(pending_delete_users::user_id)
            .do_update()
            .set(pending_delete_users::code.eq(&code))
            .execute(&mut conn)?;

        mail_recipient.do_send(SendMail {
            to: email,
            subject: "Confirm to close your Mavinote account".to_string(),
            html: CloseAccountTemplate { code: &code }.render()?,
        });

        Result::<(), HttpError>::Ok(())
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}

pub async fn close_account(
    pool: Data<Pool>,
    device: UserDevice,
    request: Sanitized<Json<CloseAccount>>,
) -> Result<Json<HttpMessage>, HttpError> {
    block(move || {
        let mut conn = pool.get().unwrap();

        let (pending_code, pending_updated_at) = pending_delete_users::table
            .filter(pending_delete_users::user_id.eq(&device.user_id))
            .select((pending_delete_users::code, pending_delete_users::updated_at))
            .first::<(String, NaiveDateTime)>(&mut conn)?;

        let minutes_since_code_sent = Utc::now()
            .naive_utc()
            .signed_duration_since(pending_updated_at)
            .num_minutes();

        if minutes_since_code_sent > 5 {
            return Err(HttpError::unprocessable_entity("expired_code"));
        }

        if pending_code != request.code {
            return Err(HttpError::unprocessable_entity("invalid_code"));
        }

        diesel::delete(user_devices::table.filter(user_devices::user_id.eq(device.user_id)))
            .execute(&mut conn)?;

        diesel::delete(users::table.filter(users::id.eq(device.user_id)))
            .execute(&mut conn)
            .map_err(|e| e.into())
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}

pub async fn listen_notifications(
    ws_server: Data<notify::ws::AddrServer>,
    device: UserDevice,
    req: HttpRequest,
    stream: Payload,
) -> Result<HttpResponse, HttpError> {
    notify::ws::start(
        notify::ws::Connection::new((&**ws_server).clone(), device.user_id, device.device_id),
        &req,
        stream,
    )
    .map_err(|e| HttpError {
        code: StatusCode::INTERNAL_SERVER_ERROR,
        error: "failed_to_start_ws",
        message: Some(format!("{}", e)),
    })
}

#[cfg(test)]
mod tests {
    use actix_web::web::Data;
    use diesel::prelude::*;
    use base::{HttpError, schema::user_devices};
    use test_helpers::db::create_pool;

    use crate::test::db::UserDeviceBuilder;

    use super::delete_device;

    #[actix_web::test]
    async fn it_returns_unknown_device_error_if_user_does_not_have_a_device_with_given_id_when_delete_device_is_called(
    ) {
        let pool = create_pool();
        let device = UserDeviceBuilder::default().email("email@email.com").pubkey("pubkey1").build(&mut pool.get().unwrap()).unwrap();
        let other_device = UserDeviceBuilder::default().email("email@email2.com").pubkey("pubkey2").build(&mut pool.get().unwrap()).unwrap();
        let request = crate::requests::DeleteDevice { id: Some(other_device.device_id) };

        let res = delete_device(Data::new(pool), device, actix_web::web::Query(request)).await;

        assert_eq!(
            HttpError::not_found("unknown_device"),
            res.unwrap_err()
        );
    }

    #[actix_web::test]
    async fn it_returns_cannot_delete_only_remaining_device_error_if_user_has_only_one_device_remaining_when_delete_device_is_called(
    ) {
        let pool = create_pool();
        let device = UserDeviceBuilder::default().build(&mut pool.get().unwrap()).unwrap();
        let request = crate::requests::DeleteDevice { id: Some(device.device_id) };

        let res = delete_device(Data::new(pool), device, actix_web::web::Query(request)).await;

        assert_eq!(
            HttpError::conflict("cannot_delete_only_remaining_device"),
            res.unwrap_err()
        );
    }

    #[actix_web::test]
    async fn it_deletes_device_from_database_when_delete_device_is_called() {
        let pool = create_pool();
        let device = UserDeviceBuilder::default().pubkey("pubkey1").build(&mut pool.get().unwrap()).unwrap();
        let device_to_delete = UserDeviceBuilder::default().user_id(device.user_id).pubkey("pubkey2").build(&mut pool.get().unwrap()).unwrap();
        let request = crate::requests::DeleteDevice { id: Some(device_to_delete.device_id) };

        let res = delete_device(Data::new(pool.clone()), device.clone(), actix_web::web::Query(request)).await;

        assert!(res.is_ok());

        let device_to_delete_exists = diesel::select(diesel::dsl::exists(
            user_devices::table.filter(user_devices::device_id.eq(&device_to_delete.device_id)),
        ))
        .get_result::<bool>(&mut pool.get().unwrap()).unwrap();

        let device_exists = diesel::select(diesel::dsl::exists(
            user_devices::table.filter(user_devices::device_id.eq(&device.device_id)),
        ))
        .get_result::<bool>(&mut pool.get().unwrap()).unwrap();

        assert!(!device_to_delete_exists);
        assert!(device_exists);
    }

    #[actix_web::test]
    async fn it_deletes_current_device_from_database_if_no_device_id_is_given_when_delete_device_is_called() {
        let pool = create_pool();
        let device = UserDeviceBuilder::default().pubkey("pubkey1").build(&mut pool.get().unwrap()).unwrap();
        UserDeviceBuilder::default().pubkey("pubkey2").user_id(device.user_id).build(&mut pool.get().unwrap()).unwrap();
        let request = crate::requests::DeleteDevice { id: None };

        let res = delete_device(Data::new(pool.clone()), device.clone(), actix_web::web::Query(request)).await;

        assert!(res.is_ok());

        let device_exists = diesel::select(diesel::dsl::exists(
            user_devices::table.filter(user_devices::device_id.eq(&device.device_id)),
        ))
        .get_result::<bool>(&mut pool.get().unwrap()).unwrap();

        assert!(!device_exists);
    }
}
