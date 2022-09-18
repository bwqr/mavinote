use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    expression::NonAggregate,
    query_builder::{QueryFragment, QueryId},
    sql_types::Text,
    types::{FromSql, HasSqlType, ToSql},
    AppearsOnTable, Expression, FromSqlRow, Queryable,
};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Folder {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub state: State,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize)]
pub struct Note {
    pub id: i32,
    pub folder_id: i32,
    pub commit: i32,
    pub title: Option<String>,
    pub text: String,
    pub state: State,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromSqlRow, Serialize)]
pub enum State {
    Clean,
    Deleted,
}

impl State {
    pub fn as_str(&self) -> &'static str {
        match self {
            State::Clean => "Clean",
            State::Deleted => "Deleted",
        }
    }
}

impl<DB> FromSql<Text, DB> for State
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&<DB>::RawValue>) -> diesel::deserialize::Result<Self> {
        let value = <String as FromSql<Text, DB>>::from_sql(bytes)?;

        match value.as_str() {
            "Clean" => Ok(State::Clean),
            "Deleted" => Ok(State::Deleted),
            _ => {
                log::error!("unrecognized value for state {value}");
                Ok(State::Clean)
            }
        }
    }
}

impl<DB> ToSql<Text, DB> for State
where
    DB: Backend,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> diesel::serialize::Result {
        <str as ToSql<Text, DB>>::to_sql(self.as_str(), out)
    }
}

impl Expression for State {
    type SqlType = Text;
}

impl<DB> QueryFragment<DB> for State
where
    DB: Backend + HasSqlType<Text>,
{
    fn walk_ast(&self, mut pass: diesel::query_builder::AstPass<DB>) -> diesel::QueryResult<()> {
        pass.push_bind_param::<Text, &'static str>(&self.as_str())
    }
}

impl AppearsOnTable<base::schemas::folders::table> for State {}
impl AppearsOnTable<base::schemas::notes::table> for State {}

impl NonAggregate for State {}

impl QueryId for State {
    type QueryId = String;

    const HAS_STATIC_QUERY_ID: bool = true;
}
