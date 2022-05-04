use actix_web::web::{ServiceConfig, scope};
use base::middlewares::auth_user::AuthUser;

mod handlers;
mod models;
mod requests;

pub fn register(config: &mut ServiceConfig) {
    config.service(
        scope("api/note")
            .wrap(AuthUser)
            .service(handlers::fetch_folders)
            .service(handlers::create_folder)
            .service(handlers::fetch_notes)
            .service(handlers::create_note)
            .service(handlers::fetch_note)
            .service(handlers::update_note)
    );
}
