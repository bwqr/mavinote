use base::middlewares::auth_user::AuthUser;

use actix_web::web::{get, post, delete, scope, ServiceConfig};

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
    );
}
