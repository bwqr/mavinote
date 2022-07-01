use actix::Actor;
use actix_cors::Cors;
use actix_web::{HttpServer, App, middleware::Logger, web::Data, http::header};
use diesel::{r2d2::{Pool as DieselPool, ConnectionManager}, PgConnection};

use base::{types::Pool, crypto::Crypto};
use notify::{SessionManager, Server};

fn setup_database() -> Pool {
    let conn_info = std::env::var("DATABASE_URL").expect("DATABASE_URL is not provided in env");
    let manager = ConnectionManager::<PgConnection>::new(conn_info);
    let pool = DieselPool::builder().build(manager).expect("Failed to create pool.");

    pool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = setup_database();
    let crypto = Crypto::new(std::env::var("SECRET_KEY").expect("SECRET_KEY is not provided in env").as_str());

    let notify_server = Server::new().start();
    let session_manager = SessionManager::new(notify_server.clone()).start();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allow_any_method()
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .allowed_header("enctype")
            .max_age(60);

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(crypto.clone()))
            .app_data(Data::new(notify_server.clone()))
            .app_data(Data::new(session_manager.clone()))
            .wrap(Logger::default())
            .configure(auth::register)
            .configure(note::register)
            .configure(notify::register)
            .configure(user::register)
    })
    .bind(std::env::var("BIND_ADDRESS").expect("APP_BIND_ADDRESS is not provided in env").as_str())?
    .run()
    .await
}
