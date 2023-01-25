use actix::Addr;
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};

use actix_web_actors::ws;
use base::{middlewares::auth_user::AuthUser, HttpError};

pub use server::Server;
pub use session::Manager as SessionManager;

use session::{messages::CreateSession, Session};

mod server;
mod session;

#[get("/connect")]
async fn connect(
    manager: web::Data<Addr<SessionManager>>,
    req: HttpRequest,
    stream: web::Payload,
) -> actix_web::error::Result<HttpResponse> {
    let session: Session = manager
        .send(CreateSession { user_id: 1 })
        .await
        .map_err(|e| {
            log::error!("failed to create session from manager {e:?}");

            HttpError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                error: "failed_to_create_session",
                message: None,
            }
        })?;

    ws::start(session, &req, stream)
}

#[post("send/{user_id}")]
async fn send_message(
    server: web::Data<Addr<Server>>,
    user_id: web::Path<i32>,
    msg: web::Json<String>,
) -> Result<String, HttpError> {
    server
        .send(server::messages::SendMessage {
            user_id: user_id.into_inner(),
            message: msg.into_inner(),
        })
        .await
        .unwrap();

    Ok("sent".to_string())
}

pub fn register(config: &mut ServiceConfig) {
    config.service(
        scope("api/notify")
            .wrap(AuthUser)
            .service(connect)
            .service(send_message),
    );
}
