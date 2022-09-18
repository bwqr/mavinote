use std::ops::Deref;

use actix_web::{FromRequest, web::Json};
use futures::{future::Map, FutureExt};

pub struct Sanitized<T: Sanitize>(pub T);

impl<T> Deref for Sanitized<T> where T: Sanitize {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Sanitize {
    fn sanitize(self) -> Self;
}

impl Sanitize for String {
    fn sanitize(self) -> Self {
        htmlescape::encode_minimal(self.trim())
    }
}

impl Sanitize for i32 {
    fn sanitize(self) -> Self {
        self
    }
}

impl<T> Sanitize for Option<T> where T: Sanitize {
    fn sanitize(self) -> Self {
        self.map(|s| s.sanitize())
    }
}

impl<T> Sanitize for Json<T> where T: Sanitize {
    fn sanitize(mut self) -> Self {
        self.0 = self.0.sanitize();

        self
    }
}

impl<T> FromRequest for Sanitized<T> where T: Sanitize + FromRequest + 'static {
    type Error = <T as FromRequest>::Error;
    type Future = Map<<T as FromRequest>::Future, fn(Result<T, Self::Error>) -> Result<Sanitized<T>, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        <T as FromRequest>::from_request(req, payload)
            .map(|res| res.map(|t| Sanitized(t.sanitize())))
    }
}
