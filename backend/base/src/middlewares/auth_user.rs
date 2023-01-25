use std::{
    future::{ready, Ready},
    task::{Context, Poll},
};

use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    web::Data,
    Error, HttpMessage, ResponseError,
};
use futures::future::LocalBoxFuture;

use crate::{crypto::Crypto, models::Token, HttpError};

const INVALID_TOKEN_ERROR: HttpError = HttpError {
    code: StatusCode::UNAUTHORIZED,
    error: "invalid_token",
    message: None,
};

const TOKEN_NOT_FOUND_ERROR: HttpError = HttpError {
    code: StatusCode::UNAUTHORIZED,
    error: "token_not_found",
    message: None,
};

const EXPIRED_TOKEN_ERROR: HttpError = HttpError {
    code: StatusCode::UNAUTHORIZED,
    error: "expired_token",
    message: None,
};

pub struct AuthUser;

impl<S, B> Transform<S, ServiceRequest> for AuthUser
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthUserMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthUserMiddleware { service }))
    }
}

pub struct AuthUserMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthUserMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match parse_token(&req) {
            Ok(token) => {
                req.extensions_mut().insert(token);
                let res = self.service.call(req);
                Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
            }
            Err(e) => Box::pin(async move {
                Ok(ServiceResponse::new(
                    req.into_parts().0,
                    e.error_response().map_into_right_body(),
                ))
            }),
        }
    }
}

fn parse_token(req: &ServiceRequest) -> Result<Token, HttpError> {
    let token = if let Some(auth_header) = req.headers().get("Authorization") {
        let auth_header = auth_header.to_str().map_err(|_e| TOKEN_NOT_FOUND_ERROR)?;

        auth_header
            .split_once("Bearer ")
            .ok_or(INVALID_TOKEN_ERROR)?
            .1
    } else {
        let query_string = req.query_string();

        // find the beginning of token
        let token = query_string
            .split_once("token=")
            .ok_or(TOKEN_NOT_FOUND_ERROR)?
            .1;

        // then find the end of the token, token can be at the end of or in the middle of query string
        token.split_once("&").map(|t| t.0).unwrap_or(token)
    };

    let crypto = req.app_data::<Data<Crypto>>().unwrap();

    crypto.decode::<Token>(token).map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => EXPIRED_TOKEN_ERROR,
        _ => INVALID_TOKEN_ERROR,
    })
}
