pub mod observable_map;

#[derive(Clone, Debug)]
pub enum State<T, E> {
    Ok(T),
    Err(E),
    Loading,
    Initial,
}

impl<T, E> Default for State<T, E> {
    fn default() -> Self {
        Self::Initial
    }
}

impl<T, E> From<Result<T, E>> for State<T, E> {
    fn from(res: Result<T, E>) -> Self {
        match res {
            Ok(ok) => Self::Ok(ok),
            Err(e) => Self::Err(e),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub api_url: String,
    pub storage_dir: String,
}
