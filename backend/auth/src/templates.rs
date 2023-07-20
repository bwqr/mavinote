use askama::Template;

#[derive(Template)]
#[template(path = "mails/verify-email.html")]
pub struct VerifyEmail<'a> {
    pub code: &'a str,
}
