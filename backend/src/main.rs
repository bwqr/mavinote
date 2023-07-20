use actix::Actor;
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool as DieselPool},
    PgConnection,
};

use base::{crypto::Crypto, types::Pool};
use notify::{ws::Server as WsServer, mail::{Server as MailServer, MailRecipient}};

fn setup_database() -> Pool {
    let conn_info = std::env::var("DATABASE_URL").expect("DATABASE_URL is not provided in env");
    let manager = ConnectionManager::<PgConnection>::new(conn_info);
    let pool = DieselPool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = setup_database();
    let crypto = Crypto::new(
        std::env::var("SECRET_KEY")
            .expect("SECRET_KEY is not provided in env")
            .as_str(),
    );

    let notify_server = WsServer::new().start();
    let mail_server: MailRecipient = MailServer::new(
            std::env::var("MAIL_ADDRESS").expect("MAIL_ADDRESS is not provided in env"),
            std::env::var("MAILGUN_ENDPOINT").expect("MAILGUN_ENDPOINT is not provided in env"),
            std::env::var("MAILGUN_KEY").expect("MAILGUN_KEY is not provided in env"),
        )
        .start()
        .recipient();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(
                std::env::var("CORS_ORIGIN")
                    .expect("CORS_ORIGIN is not provided in env")
                    .as_str(),
            )
            .allow_any_method()
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .allowed_header("enctype")
            .max_age(60);

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(crypto.clone()))
            .app_data(Data::new(notify_server.clone()))
            .app_data(Data::new(mail_server.clone()))
            .wrap(Logger::default())
            .configure(auth::register)
            .configure(note::register)
            .configure(user::register)
    })
    .bind(
        std::env::var("BIND_ADDRESS")
            .expect("APP_BIND_ADDRESS is not provided in env")
            .as_str(),
    )?
    .run()
    .await
}
