use std::sync::Arc;

use base::{Error, Config, models::Token};
use requests::SignUp;
use reqwest::{Client, StatusCode};

use crate::requests::Login;

mod requests;

pub async fn login(email: &str, password: &str) -> Result<Token, Error> {
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let email = email.trim();
    let password = password.trim();

    let request_body = serde_json::to_string(&Login { email, password }).unwrap();

    client
        .post(format!("{}/auth/login", config.api_url))
        .body(request_body)
        .send()
        .await?
        .error_for_status()?
        .json::<Token>()
        .await
        .map_err(|e| e.into())
}

pub async fn sign_up(name: &str, email: &str, password: &str) -> Result<Token, Error> {
    let client = runtime::get::<Arc<Client>>().unwrap();
    let config = runtime::get::<Arc<Config>>().unwrap();

    let name = name.trim();
    let email = email.trim();
    let password = password.trim();

    let request_body = serde_json::to_string(&SignUp { name, email, password }).unwrap();

    let response = client
        .post(format!("{}/auth/sign-up", config.api_url))
        .body(request_body)
        .send()
        .await?;

    if response.status() == StatusCode::CONFLICT {
        return Err(Error::Message("user_with_given_email_already_exists".to_string()))
    }

    response
        .error_for_status()?
        .json::<Token>()
        .await
        .map_err(|e| e.into())
}
