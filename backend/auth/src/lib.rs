use base::HttpError;

use actix_web::{web, http::StatusCode};

mod handlers;
mod requests;
mod responses;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("api/auth")
            .service(handlers::login)
            .service(handlers::sign_up)
    );
}

pub(crate) enum Error {
    InvalidCredentials,
    UserExists,
}

impl Into<HttpError> for Error {
    fn into(self) -> HttpError {
        match self {
            Error::InvalidCredentials => HttpError {
                code: StatusCode::UNAUTHORIZED,
                error: "invalidCredentials",
                message: None,
            },
            Error::UserExists => HttpError {
                code: StatusCode::CONFLICT,
                error: "userExists",
                message: None,
            },
        }
    }
}
