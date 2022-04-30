use actix_web::{HttpServer, App, middleware::Logger, web::Data};
use diesel::{r2d2::{Pool as DieselPool, ConnectionManager}, PgConnection};

use base::{types::Pool, crypto::Crypto};

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

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(crypto.clone()))
            .wrap(Logger::default())
            .configure(auth::register)
            .configure(note::register)
            .configure(user::register)
    })
    .bind(std::env::var("BIND_ADDRESS").expect("APP_BIND_ADDRESS is not provided in env").as_str())?
    .run()
    .await
}
