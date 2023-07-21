use actix_web::web::{post, scope, ServiceConfig, get};
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
            .route("folder/{folder_id}", get().to(handlers::fetch_folder))
            .service(handlers::create_folder)
            .service(handlers::delete_folder)
            .service(handlers::fetch_note)
            .service(handlers::create_note)
            .service(handlers::update_note)
            .service(handlers::delete_note)
            .service(handlers::fetch_requests)
            .route("requests", post().to(handlers::create_requests))
            .service(handlers::respond_requests),
    );
}
