pub mod db {
    use base::types::Pool;

    use diesel::{prelude::*, r2d2::ConnectionManager, PgConnection};

    pub fn create_pool() -> Pool {
        let conn_info = "postgres://mavinote:toor@127.0.0.1/mavinote_test";
        let manager = ConnectionManager::<PgConnection>::new(conn_info);

        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        pool.get().unwrap().begin_test_transaction().unwrap();

        pool
    }
}
