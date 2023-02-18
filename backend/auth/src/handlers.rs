use base::{
    crypto::Crypto,
    models::{Token, TokenKind, UNEXPECTED_TOKEN_KIND},
    sanitize::Sanitized,
    schema::{devices, pending_devices, pending_users, users},
    types::Pool,
    HttpError, HttpMessage,
};

use actix_web::{
    http::StatusCode,
    post,
    web::{block, Data, Json, Payload, Query},
    HttpRequest, HttpResponse,
};
use base64::prelude::{Engine, BASE64_STANDARD};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rand::seq::SliceRandom;

use crate::{
    models::PendingUser,
    requests::{CreatePendingDevice, Login, SendCode, SignUp},
    responses,
};

pub async fn login(
    crypto: Data<Crypto>,
    pool: Data<Pool>,
    request: Json<Login>,
) -> Result<Json<responses::Token>, HttpError> {
    let token = block(move || -> Result<String, HttpError> {
        let device_id = devices::table
            .filter(devices::pubkey.eq(&request.pubkey))
            .filter(devices::password.eq(crypto.sign512(&request.password)))
            .filter(users::email.eq(&request.email))
            .inner_join(users::table)
            .select(devices::id)
            .first::<i32>(&mut pool.get().unwrap())
            .map_err(|e| match e {
                diesel::result::Error::NotFound => HttpError {
                    code: StatusCode::UNAUTHORIZED,
                    error: "invalid_credentials",
                    message: None,
                },
                e => e.into(),
            })?;

        crypto
            .encode(&Token::device(device_id))
            .map_err(|e| e.into())
    })
    .await??;

    Ok(Json(responses::Token { token }))
}

pub async fn sign_up(
    crypto: Data<Crypto>,
    pool: Data<Pool>,
    request: Sanitized<Json<SignUp>>,
) -> Result<Json<responses::Token>, HttpError> {
    match BASE64_STANDARD.decode(&request.pubkey) {
        Ok(bytes) if bytes.len() == 32 => {}
        _ => return Err(HttpError::unprocessable_entity("invalid_pubkey")),
    };

    if request.password.len() < 32 || request.password.len() > 64 {
        return Err(HttpError::unprocessable_entity("invalid_password"));
    }

    let token = block(move || -> Result<String, HttpError> {
        let mut conn = pool.get().unwrap();

        let pending_user = pending_users::table
            .filter(pending_users::email.eq(&request.email))
            .first::<PendingUser>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => HttpError::not_found("email_not_found"),
                _ => e.into(),
            })?;

        let minutes_since_code_sent = Utc::now()
            .naive_utc()
            .signed_duration_since(pending_user.updated_at)
            .num_minutes();

        if minutes_since_code_sent > 5 {
            return Err(HttpError::unprocessable_entity("code_expired"));
        }

        if pending_user.code != request.code {
            return Err(HttpError::unprocessable_entity("invalid_code"));
        }

        let (user_id, _, _) = diesel::insert_into(users::table)
            .values(users::email.eq(pending_user.email))
            .get_result::<(i32, String, NaiveDateTime)>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => HttpError::conflict("user_already_exists"),
                _ => e.into(),
            })?;

        let device_id = diesel::insert_into(devices::table)
            .values((
                devices::user_id.eq(user_id),
                devices::pubkey.eq(&request.pubkey),
                devices::password.eq(crypto.sign512(&request.password)),
            ))
            .get_result::<(i32, i32, String, String)>(&mut conn)?
            .0;

        diesel::delete(pending_users::table)
            .filter(pending_users::email.eq(&request.email))
            .execute(&mut conn)?;

        crypto
            .encode(&Token::device(device_id))
            .map_err(|e| e.into())
    })
    .await??;

    Ok(Json(responses::Token { token }))
}

#[post("send-code")]
pub async fn send_code(
    pool: Data<Pool>,
    request: Sanitized<Json<SendCode>>,
) -> Result<Json<HttpMessage>, HttpError> {
    block(move || -> Result<(), HttpError> {
        let mut conn = pool.get().unwrap();

        let user_exists = diesel::dsl::select(diesel::dsl::exists(
            users::table.filter(users::email.eq(&request.email)),
        ))
        .get_result::<bool>(&mut conn)?;

        if user_exists {
            return Err(HttpError::unprocessable_entity("user_exists"));
        }

        let code: String = b"0123456789"
            .choose_multiple(&mut rand::thread_rng(), 8)
            .map(|num| char::from(*num))
            .collect();

        diesel::insert_into(pending_users::table)
            .values((
                pending_users::code.eq(&code),
                pending_users::email.eq(&request.email),
            ))
            .on_conflict(pending_users::email)
            .do_update()
            .set(pending_users::code.eq(&code))
            .execute(&mut conn)?;

        Ok(())
    })
    .await??;

    Ok(Json(HttpMessage::success()))
}

#[post("request-verification")]
pub async fn create_pending_device(
    crypto: Data<Crypto>,
    pool: Data<Pool>,
    request: Sanitized<Json<CreatePendingDevice>>,
) -> Result<Json<responses::Token>, HttpError> {
    match BASE64_STANDARD.decode(&request.pubkey) {
        Ok(bytes) if bytes.len() == 32 => {}
        _ => return Err(HttpError::unprocessable_entity("invalid_pubkey")),
    };

    if request.password.len() < 32 || request.password.len() > 64 {
        return Err(HttpError::unprocessable_entity("invalid_password"));
    }

    let token = block(move || {
        let mut conn = pool.get().unwrap();

        let user_exists = diesel::select(diesel::dsl::exists(
            users::table.filter(users::email.eq(&request.email)),
        ))
        .get_result::<bool>(&mut conn)?;

        if !user_exists {
            return Err(HttpError::unprocessable_entity("email_not_found"));
        }

        let password = crypto.sign512(&request.password);

        let existing_dev_pass = devices::table
            .filter(devices::pubkey.eq(&request.pubkey))
            .filter(users::email.eq(&request.email))
            .inner_join(users::table)
            .select(devices::password)
            .first::<String>(&mut conn)
            .optional()?;

        if let Some(pass) = existing_dev_pass {
            return if pass == password {
                Err(HttpError::conflict("device_already_exists"))
            } else {
                Err(HttpError::conflict("device_exists_but_password_mismatch"))
            };
        }

        let (pending_device_id, _, _, _, _) = diesel::insert_into(pending_devices::table)
            .values((
                pending_devices::email.eq(&request.email),
                pending_devices::pubkey.eq(&request.pubkey),
                pending_devices::password.eq(&password),
            ))
            .on_conflict((pending_devices::email, pending_devices::pubkey))
            .do_update()
            .set(pending_devices::password.eq(&password))
            .get_result::<(i32, String, String, String, NaiveDateTime)>(&mut conn)?;

        crypto
            .encode(&Token::pending_device(pending_device_id))
            .map_err(|e| e.into())
    })
    .await??;

    Ok(Json(responses::Token { token }))
}

pub async fn wait_verification(
    pool: Data<Pool>,
    crypto: Data<Crypto>,
    ws_server: Data<notify::ws::AddrServer>,
    req: HttpRequest,
    stream: Payload,
    query: Query<responses::Token>,
) -> Result<HttpResponse, HttpError> {
    let token = crypto.decode::<Token>(&query.token)?;

    if token.kind != TokenKind::PendingDevice {
        return Err(UNEXPECTED_TOKEN_KIND);
    }

    let pending_device_id = block(move || {
        pending_devices::table
            .filter(pending_devices::id.eq(token.id))
            .select(pending_devices::id)
            .first(&mut pool.get().unwrap())
    })
    .await??;

    notify::ws::start(
        notify::ws::Connection::new((&**ws_server).clone(), pending_device_id),
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
    use actix_web::web::{Data, Json};
    use base::{
        crypto::Crypto,
        sanitize::Sanitized,
        schema::{devices, pending_users, users},
        types::Pool,
        HttpError,
    };
    use base64::prelude::{Engine, BASE64_STANDARD};
    use diesel::{prelude::*, r2d2::ConnectionManager, PgConnection};

    use crate::requests;

    use super::sign_up;

    fn create_pool() -> Pool {
        let conn_info = "postgres://mavinote:toor@127.0.0.1/mavinote_test";
        let manager = ConnectionManager::<PgConnection>::new(conn_info);

        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        pool.get().unwrap().begin_test_transaction().unwrap();

        pool
    }

    #[actix_web::test]
    async fn it_returns_invalid_pubkey_error_if_pubkey_is_not_base64_encoded_valid_pubkey_when_sign_up_is_called(
    ) {
        let pool = create_pool();

        let request = requests::SignUp {
            email: "".to_string(),
            code: "".to_string(),
            pubkey: "".to_string(),
            password: "".to_string(),
        };

        let res = sign_up(
            Data::new(Crypto::new("SECRET")),
            Data::new(pool),
            Sanitized(Json(request)),
        )
        .await;

        assert_eq!(
            HttpError::unprocessable_entity("invalid_pubkey"),
            res.map(|_| ()).unwrap_err()
        );
    }

    #[actix_web::test]
    async fn it_returns_email_not_found_error_if_there_is_no_pending_users_row_for_given_email_when_sign_up_is_called(
    ) {
        let pool = create_pool();

        let request = requests::SignUp {
            email: "".to_string(),
            code: "".to_string(),
            pubkey: BASE64_STANDARD.encode([0; 32]),
            password: "1234".repeat(10),
        };

        let res = sign_up(
            Data::new(Crypto::new("SECRET")),
            Data::new(pool),
            Sanitized(Json(request)),
        )
        .await;

        assert_eq!(
            HttpError::not_found("email_not_found"),
            res.map(|_| ()).unwrap_err()
        );
    }

    #[actix_web::test]
    async fn it_returns_invalid_password_error_if_password_is_not_between_32_and_64_char_when_sign_up_is_called(
    ) {
        let pool = create_pool();

        diesel::insert_into(pending_users::table)
            .values((
                pending_users::email.eq("EMAIL"),
                pending_users::code.eq("11223344"),
            ))
            .execute(&mut pool.get().unwrap())
            .unwrap();

        let short_password_req = requests::SignUp {
            email: "EMAIL".to_string(),
            code: "11223344".to_string(),
            pubkey: BASE64_STANDARD.encode([0; 32]),
            password: "1234567890".repeat(3),
        };

        let res = sign_up(
            Data::new(Crypto::new("SECRET")),
            Data::new(pool.clone()),
            Sanitized(Json(short_password_req)),
        )
        .await;

        assert_eq!(
            HttpError::unprocessable_entity("invalid_password"),
            res.map(|_| ()).unwrap_err()
        );

        let long_password_req = requests::SignUp {
            email: "EMAIL".to_string(),
            code: "11223344".to_string(),
            pubkey: BASE64_STANDARD.encode([0; 32]),
            password: "1234567890".repeat(7),
        };

        let res = sign_up(
            Data::new(Crypto::new("SECRET")),
            Data::new(pool),
            Sanitized(Json(long_password_req)),
        )
        .await;

        assert_eq!(
            HttpError::unprocessable_entity("invalid_password"),
            res.map(|_| ()).unwrap_err()
        );
    }

    #[actix_web::test]
    async fn it_returns_invalid_code_error_if_the_code_for_given_email_is_incorrect_when_sign_up_is_called(
    ) {
        let pool = create_pool();

        diesel::insert_into(pending_users::table)
            .values((
                pending_users::email.eq("EMAIL"),
                pending_users::code.eq("11223344"),
            ))
            .execute(&mut pool.get().unwrap())
            .unwrap();

        let request = requests::SignUp {
            email: "EMAIL".to_string(),
            code: "44332211".to_string(),
            pubkey: BASE64_STANDARD.encode([0; 32]),
            password: "1234".repeat(10),
        };

        let res = sign_up(
            Data::new(Crypto::new("SECRET")),
            Data::new(pool),
            Sanitized(Json(request)),
        )
        .await;

        assert_eq!(
            HttpError::unprocessable_entity("invalid_code"),
            res.map(|_| ()).unwrap_err()
        );
    }

    #[actix_web::test]
    async fn it_creates_user_and_device_when_sign_up_is_called() {
        let pool = create_pool();

        diesel::insert_into(pending_users::table)
            .values((
                pending_users::email.eq("EMAIL"),
                pending_users::code.eq("11223344"),
            ))
            .execute(&mut pool.get().unwrap())
            .unwrap();

        let request = requests::SignUp {
            email: "EMAIL".to_string(),
            code: "11223344".to_string(),
            pubkey: BASE64_STANDARD.encode([1; 32]),
            password: "PASSWORD".repeat(4),
        };

        let res = sign_up(
            Data::new(Crypto::new("SECRET")),
            Data::new(pool.clone()),
            Sanitized(Json(request)),
        )
        .await;

        assert!(res.is_ok());

        let user = users::table
            .filter(users::email.eq("EMAIL"))
            .select((users::id, users::email))
            .first::<(i32, String)>(&mut pool.get().unwrap());

        assert!(user.is_ok());

        let device = devices::table
            .filter(devices::user_id.eq(user.unwrap().0))
            .filter(devices::pubkey.eq(BASE64_STANDARD.encode([1; 32])))
            .select(devices::id)
            .first::<i32>(&mut pool.get().unwrap());

        assert!(device.is_ok());
    }
}
