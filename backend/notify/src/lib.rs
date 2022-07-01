use actix::Addr;
use actix_web::{web::{ServiceConfig, scope, self}, HttpRequest, HttpResponse, get, post, http::StatusCode};

use actix_web_actors::ws;
use base::{HttpError, middlewares::auth_user::AuthUser};
use user::models::User;

pub use session::Manager as SessionManager;
pub use server::Server;

use session::{Session, messages::CreateSession};

mod server;
mod session;

#[get("/connect")]
async fn connect(manager: web::Data<Addr<SessionManager>>, req: HttpRequest, stream: web::Payload) -> actix_web::error::Result<HttpResponse> {
    let session: Session = manager.send(CreateSession { user_id: 1 })
        .await
        .map_err(|e| {
            log::error!("failed to create session from manager {e:?}");

            HttpError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                error: "failedToCreateSession",
                message: None,
            }
        })?;

    ws::start(session, &req, stream)
}

#[post("send/{user_id}")]
async fn send_message(server: web::Data<Addr<Server>>, user_id: web::Path<i32>, msg: web::Json<String>) -> Result<String, HttpError> {
    server.send(server::messages::SendMessage { user_id: user_id.into_inner(), message: msg.into_inner()})
        .await
        .unwrap();

    Ok("sent".to_string())
}

pub fn register(config: &mut ServiceConfig) {
    config.service(
        scope("api/notify")
            //.wrap(AuthUser)
            .service(connect)
            .service(send_message)
    );
}
