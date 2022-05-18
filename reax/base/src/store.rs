use std::{pin::Pin, future::Future};

pub trait Store where Self: Sync + Send {
    fn get<'a>(&'a self, key: &'a str) -> Pin<Box<dyn Future<Output = Result<Option<String>, crate::Error>> + Send + 'a>>;

    fn put<'a>(&'a self, key: &'a str, value: &'a str) -> Pin<Box<dyn Future<Output = Result<(), crate::Error>> + Send + 'a>>;
}
