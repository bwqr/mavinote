use actix::Addr;
use actix_web::{web::{ServiceConfig, scope, self}, HttpRequest, HttpResponse, error::Result, get, http::StatusCode};

use actix_web_actors::ws;
use base::{HttpError, middlewares::auth_user::AuthUser};
use user::models::User;

pub use session::Manager as SessionManager;
pub use server::Server;

use session::{Session, messages::CreateSession};

mod server;
mod session;

#[get("/connect")]
pub async fn connect(manager: web::Data<Addr<SessionManager>>, req: HttpRequest, stream: web::Payload, user: User) -> Result<HttpResponse> {
    let session: Session = manager.send(CreateSession { user_id: user.id })
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
pub fn register(config: &mut ServiceConfig) {
    config.service(
        scope("api/notify")
            .wrap(AuthUser)
            .service(connect)
    );
}
