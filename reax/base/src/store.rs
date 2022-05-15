#[derive(Debug)]
pub struct Store;

impl Store {
    pub async fn get(&self, key: &str) -> Result<Option<String>, crate::Error> {
        Ok(Some("Hello".to_string()))
    }

    pub async fn put(&self, key: &str, value: &str) -> Result<(), crate::Error> {
        Ok(())
    }
}
