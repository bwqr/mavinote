use diesel::{r2d2::{Pool as DieselPool, ConnectionManager}, PgConnection};

pub type Pool = DieselPool<ConnectionManager<PgConnection>>;
