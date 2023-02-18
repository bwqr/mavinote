use actix_web::web::{get, post, scope, ServiceConfig};

mod handlers;
mod models;
mod requests;
mod responses;

pub fn register(config: &mut ServiceConfig) {
    config.service(
        scope("api/auth")
            .route("sign-up", post().to(handlers::sign_up))
            .route("login", post().to(handlers::login))
            .route("send-code", post().to(handlers::send_code))
            .route("request-verification", post().to(handlers::request_verification))
            .route("wait-verification", get().to(handlers::wait_verification)),
    );
}
