use std::sync::Mutex;

use futures_util::StreamExt;
use once_cell::sync::OnceCell;
use tokio::task::JoinHandle;
use tokio_tungstenite::{connect_async, tungstenite};
use tokio::sync::watch::{channel, Sender};

static CONNECTION: OnceCell<Mutex<Connection>> = OnceCell::new();
static CONNECTED: OnceCell<Sender<bool>> = OnceCell::new();

pub fn init(ws_url: String) {
    CONNECTION
        .set(Mutex::new(Connection {
            ws_url,
            join_handle: None,
        }))
        .unwrap();

    CONNECTED.set(channel(false).0).unwrap();
}

pub fn connected() -> tokio::sync::watch::Receiver<bool> {
    CONNECTED.get().unwrap().subscribe()
}

pub async fn start() {
    CONNECTION.get().unwrap().lock().unwrap().start();
}

pub async fn stop() {
    CONNECTION.get().unwrap().lock().unwrap().stop();
}

#[derive(Debug)]
struct Connection {
    ws_url: String,
    join_handle: Option<JoinHandle<()>>,
}

impl Connection {
    pub fn start(&mut self) {
        if self.join_handle.is_some() {
            log::debug!("There exists already one connection, cannot start another one");
            return;
        }

        let ws_url = self.ws_url.clone();

        self.join_handle = Some(tokio::spawn(async move {
            loop {
                if let Err(e) = Self::connect(ws_url.as_str()).await {
                    log::debug!("connect is failed, {e:?}");
                }

                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }));
    }

    pub fn stop(&mut self) {
        if let Some(handle) = &self.join_handle.take() {
            CONNECTED.get().unwrap().send_replace(false);
            handle.abort();
        } else {
            log::debug!("cannot stop nonexistent connection");
        }
    }

    async fn connect(ws_url: &str) -> Result<(), tungstenite::Error> {
        let (mut sock, _) = connect_async(ws_url).await?;

        log::debug!("connection is established");
        CONNECTED.get().unwrap().send_replace(true);

        while let Some(msg) = sock.next().await {
            match msg {
                Ok(msg) => log::debug!("msg is received, {}", msg.into_text()?),
                Err(e) => log::debug!("error on next, {e:?}"),
            };
        }

        log::debug!("connection is closed");
        CONNECTED.get().unwrap().send_replace(false);

        Ok(())
    }
}
