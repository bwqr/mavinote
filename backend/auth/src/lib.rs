use actix_web::web;

mod handlers;
mod models;
mod requests;
mod responses;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("api/auth")
            .service(handlers::sign_up)
            .service(handlers::send_code),
    );
}
