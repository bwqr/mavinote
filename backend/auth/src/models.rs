use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Queryable)]
pub struct PendingUser {
    pub code: String,
    pub email: String,
    pub updated_at: NaiveDateTime,
}
