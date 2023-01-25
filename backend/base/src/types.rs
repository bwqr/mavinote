use diesel::{
    r2d2::{ConnectionManager, Pool as DieselPool},
    PgConnection,
};

pub type Pool = DieselPool<ConnectionManager<PgConnection>>;
