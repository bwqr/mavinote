use askama::Template;

#[derive(Template)]
#[template(path = "mails/close-account.html")]
pub struct CloseAccount<'a> {
    pub code: &'a str,
}
