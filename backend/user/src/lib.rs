use base::middlewares::auth_user::AuthUser;

use actix_web::web::{delete, get, post, put, scope, ServiceConfig};

mod handlers;
pub mod models;
mod requests;
pub mod test;

use serde::Deserialize;
#[derive(Deserialize)]
struct Req {
    pub user_id: i32,
    pub device_id: i32,
    pub message: String,
}

async fn send_notification(
    ws_server: actix_web::web::Data<notify::ws::AddrServer>,
    req: actix_web::web::Json<Req>,
) -> String {

    ws_server.do_send(notify::ws::messages::SendDeviceMessage {
        user_id: req.user_id,
        device_id: req.device_id,
        message: notify::ws::messages::DeviceMessage::Text(req.message.clone()),
    });
    req.message.clone()
}

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
            .route("close", put().to(handlers::close_account))
            .route("notifications", get().to(handlers::listen_notifications))
    );

    config.service(
        scope("api/not")
            .route("send", post().to(send_notification))
    );
}
