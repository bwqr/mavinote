use std::str::FromStr;

use iced::{Application, Column, Command, Container, Settings};

use reqwest::{Client, header::{HeaderMap, HeaderValue}, ClientBuilder};
use once_cell::sync::OnceCell;
use sqlx::{Sqlite, Pool, sqlite::{SqliteConnectOptions, SqlitePoolOptions}};

use base::{Config, Store};

use ui::note::folders::{Folders, Message as FoldersMessage};

static CONFIG: OnceCell<Config> = OnceCell::new();
static DATABASE: OnceCell<Pool<Sqlite>> = OnceCell::new();
static CLIENT: OnceCell<Client> = OnceCell::new();
static STORE: OnceCell<Store> = OnceCell::new();

mod ui;

#[tokio::main]
async fn main() -> iced::Result {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let config = Config {
        api_url: "http://127.0.0.1:8050/api".to_string(),
        storage_dir: "./".to_string(),
    };

    let db_path = format!("sqlite:{}/app.db", config.storage_dir);

    let options = SqliteConnectOptions::from_str(db_path.as_str())
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .unwrap();

    sqlx::migrate!("../reax/migrations").run(&pool).await.unwrap();


    DATABASE.set(pool).expect("failed to set database");

    STORE.set(Store::new(DATABASE.get().unwrap())).expect("failed to set database");

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();


    CLIENT.set(client).expect("failed to set client");

    CONFIG.set(config).expect("failed to set config");

    MaviNote::run(Settings::default())
}

enum Page {
    Folders(ui::note::folders::Folders),
}

struct MaviNote {
    page: Page,
}

#[derive(Debug)]
enum Message {
    FoldersMessage(FoldersMessage),
}

impl Application for MaviNote {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (folders, command) = Folders::new();
        (
            Self {
                page: Page::Folders(folders),
            },
            command.map(Message::FoldersMessage),
        )
    }

    fn title(&self) -> String {
        "MaviNote".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match (message, &mut self.page) {
            (Message::FoldersMessage(m), Page::Folders(folders)) => folders.update(m)
        };

        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        match &mut self.page {
            Page::Folders(folders) => {
                let folders = Column::new().push(
                    folders
                        .view()
                        .map(|message| Message::FoldersMessage(message)),
                );

                Container::new(folders).into()
            }
        }
    }
}

pub(crate) fn pool() -> &'static Pool<Sqlite> {
    DATABASE.get().unwrap()
}

pub(crate) fn client() -> &'static Client {
    CLIENT.get().unwrap()
}

pub(crate) fn config() -> &'static Config {
    CONFIG.get().unwrap()
}

pub(crate) fn store() -> &'static Store {
    STORE.get().unwrap()
}
