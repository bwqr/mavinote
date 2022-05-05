use serde::Serialize;

#[derive(Serialize)]
pub struct Login<'a> {
    pub email: &'a str,
    pub password: &'a str,
}
