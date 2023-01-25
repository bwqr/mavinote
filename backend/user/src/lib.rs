use base::middlewares::auth_user::AuthUser;

use actix_web::web::{scope, ServiceConfig};

mod handlers;
pub mod models;
mod requests;

pub fn register(config: &mut ServiceConfig) {
    config.service(
        scope("api/user")
            .wrap(AuthUser)
            .service(handlers::fetch_devices),
    );
}
