use base::middlewares::auth_user::AuthUser;

use actix_web::web::{scope, ServiceConfig};

mod handlers;
pub mod models;
mod requests;
mod responses;

pub fn register(config: &mut ServiceConfig) {
    config.service(
        scope("api/user")
            .wrap(AuthUser)
            .service(handlers::fetch_devices)
            .service(handlers::add_device)
    );
}
