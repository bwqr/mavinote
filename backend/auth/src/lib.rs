use actix_web::web::{post, scope, ServiceConfig};

mod handlers;
mod models;
mod requests;
mod responses;

pub fn register(config: &mut ServiceConfig) {
    config.service(
        scope("api/auth")
            .route("sign-up", post().to(handlers::sign_up))
            .service(handlers::send_code)
            .service(handlers::create_pending_device),
    );
}
