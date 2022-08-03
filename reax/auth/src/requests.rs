use serde::Serialize;

#[derive(Serialize)]
pub struct Login<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Serialize)]
pub struct SignUp<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
