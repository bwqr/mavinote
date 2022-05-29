use std::{sync::{Arc, RwLock}, collections::HashMap};

type Sender<T> = tokio::sync::watch::Sender<T>;

pub struct Receiver<T> {
    store: Arc<ObservableMap<T>>,
    key: i32,
    receiver: tokio::sync::watch::Receiver<T>,
}

impl<T> Receiver<T> {
    pub fn inner<'a>(&'a mut self) -> &'a mut tokio::sync::watch::Receiver<T> {
        &mut self.receiver
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.store.unsubscribe(self.key);
    }
}

#[derive(Debug)]
pub struct ObservableMap<T>(RwLock<HashMap<i32, Sender<T>>>);

impl<T> ObservableMap<T> {
    pub fn new() -> Self {
        Self(RwLock::new(HashMap::new()))
    }

    pub fn contains_key(&self, key: i32) -> bool {
        self.0.read().unwrap().contains_key(&key)
    }

    pub fn insert(&self, key: i32, value: T) {
        let (tx, _) = tokio::sync::watch::channel(value);
        if self.0.write().unwrap().insert(key, tx).is_some() {
            log::warn!("key is overwritten {key}");
        }

        log::debug!("inserted key {key}");
    }

    pub fn update(&self, key: i32, value: T) {
        self.update_modify(key, |old| *old = value);
    }

    pub fn update_modify<F: FnOnce(&mut T) -> ()>(&self, key: i32, func: F) {
        if let Some(sender) = self.0.read().unwrap().get(&key) {
            sender.send_modify(func);
        }
    }

    pub fn subscribe(self: &Arc<Self>, key: i32) -> Option<Receiver<T>> {
        if let Some(sender) = self.0.read().unwrap().get(&key) {
            Some(Receiver {
                store: self.clone(),
                key,
                receiver: sender.subscribe(),
            })
        } else {
            None
        }
    }

    pub fn unsubscribe(&self, key: i32) {
        let mut map = self.0.write().unwrap();

        if let Some(sender) = map.get(&key) {
            if sender.receiver_count() == 1 {
                map.remove(&key);

                log::debug!("removed key from store {key}");
            }
        }
    }
}

