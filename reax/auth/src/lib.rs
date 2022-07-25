use std::sync::Arc;

use base::{Error, Config, models::Token};
use reqwest::{Client, StatusCode};

use crate::requests::Login;

mod requests;

pub async fn login(email: &str, password: &str) -> Result<Token, Error> {
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let email = email.trim();
    let password = password.trim();

    if email.is_empty() || password.is_empty() {
        return Err(Error::Message("Email and password must be filled out".to_string()));
    }

    let request_body = serde_json::to_string(&Login { email, password }).unwrap();

    let response = client
        .post(format!("{}/auth/login", config.api_url))
        .body(request_body)
        .send()
        .await?;

    if StatusCode::UNAUTHORIZED == response.status() {
        return Err(Error::Message("Email or password is invalid".to_string()));
    }

    response
        .error_for_status()?
        .json::<Token>()
        .await
        .map_err(|e| e.into())
}
