use base::{Error, Config, models::Token, Store};
use reqwest::{Client, StatusCode};

use crate::requests::Login;

mod requests;

pub async fn login(store: &'static Store, client: &'static Client, config: &'static Config, email: String, password: String) -> Result<(), Error> {
    let email = email.as_str().trim();
    let password = password.as_str().trim();

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

    let token = response
        .error_for_status()?
        .json::<Token>()
        .await?;

    if let Err(e) = store.put("token", token.token.as_str()).await {
        log::error!("failed to set token, {:?}", e);
    }

    log::debug!("received token {}", token.token);
    Ok(())
}
