use actix_web::web::{scope, ServiceConfig};
use base::middlewares::auth_user::AuthUser;

mod handlers;
mod models;
mod requests;
mod responses;

pub fn register(config: &mut ServiceConfig) {
    config.service(
        scope("api/note")
            .wrap(AuthUser)
            .service(handlers::fetch_folders)
            .service(handlers::create_folder)
            .service(handlers::delete_folder)
            .service(handlers::fetch_commits)
            .service(handlers::fetch_note)
            .service(handlers::create_note)
            .service(handlers::update_note)
            .service(handlers::delete_note),
    );
}
