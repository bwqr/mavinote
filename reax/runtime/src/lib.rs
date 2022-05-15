use std::sync::{RwLock};

use anymap::any::Any;
use base::Data;
use once_cell::sync::OnceCell;

struct Runtime {
    types: anymap::Map<dyn Any + Sync + Send>,
}

static RUNTIME: OnceCell<RwLock<Runtime>> = OnceCell::new();

pub fn init() {
    RUNTIME
        .set(RwLock::new(Runtime {
            types: anymap::Map::new(),
        }))
        .map_err(|_| "Failed to set runtime")
        .unwrap();
}

pub fn get<T: Send + Sync + 'static>() -> Option<Data<T>> {
    let runtime = RUNTIME.get().unwrap().read().unwrap();

    runtime.types.get::<Data<T>>().map(|value| (*value).clone())
}

pub fn put<T: Send + Sync + 'static>(value: T) {
    RUNTIME.get().unwrap().write().unwrap().types.insert(Data::new(value));
}
