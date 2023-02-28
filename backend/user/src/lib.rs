use base::middlewares::auth_user::AuthUser;

use actix_web::web::{delete, get, post, put, scope, ServiceConfig};

mod handlers;
pub mod models;
mod requests;

pub fn register(config: &mut ServiceConfig) {
    config.service(
        scope("api/user")
            .wrap(AuthUser)
            .route("devices", get().to(handlers::fetch_devices))
            .route("device", post().to(handlers::add_device))
            .route("device", delete().to(handlers::delete_device))
            .route(
                "send-close-code",
                post().to(handlers::send_close_account_code),
            )
            .route("close", put().to(handlers::close_account)),
    );
}
