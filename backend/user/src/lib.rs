use base::middlewares::auth_user::AuthUser;

use actix_web::web::{ServiceConfig, scope};

mod handlers;
pub mod models;

pub fn register(config: &mut ServiceConfig) {
    config.service(
        scope("api/user")
            .wrap(AuthUser)
            .service(handlers::profile)
    );
}
