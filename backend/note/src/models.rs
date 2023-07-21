use chrono::NaiveDateTime;
use diesel::{
    backend::RawValue,
    deserialize::{self, FromSql},
    pg::Pg,
    serialize::{self, ToSql},
    AsExpression, FromSqlRow, Queryable,
};
use serde::Serialize;
use std::io::Write;

#[derive(Queryable, Serialize)]
pub struct Folder {
    pub id: i32,
    pub user_id: i32,
    pub state: State,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize)]
pub struct Note {
    pub id: i32,
    pub folder_id: i32,
    pub commit: i32,
    pub state: State,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsExpression, Clone, Debug, FromSqlRow, Serialize)]
#[diesel(sql_type = base::schema::sql_types::State)]
pub enum State {
    Clean,
    Deleted,
}

impl FromSql<base::schema::sql_types::State, Pg> for State {
    fn from_sql(value: RawValue<Pg>) -> deserialize::Result<Self> {
        let bytes = value.as_bytes();

        match bytes {
            b"Clean" => Ok(State::Clean),
            b"Deleted" => Ok(State::Deleted),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<base::schema::sql_types::State, Pg> for State {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            State::Clean => out.write_all(b"Clean")?,
            State::Deleted => out.write_all(b"Deleted")?,
        }
        Ok(serialize::IsNull::No)
    }
}
