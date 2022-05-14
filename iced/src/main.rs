use std::str::FromStr;

use iced::{Application, Column, Command, Container, Settings};

use reqwest::{Client, header::{HeaderMap, HeaderValue}, ClientBuilder};
use once_cell::sync::OnceCell;
use sqlx::{Sqlite, Pool, sqlite::{SqliteConnectOptions, SqlitePoolOptions}};

use base::{Config, Store};

use ui::note::{folders::{Folders, Message as FoldersMessage}, note::Note};
use ui::note::notes::{Notes, Message as NotesMessage};
use ui::note::note::Message as NoteMessage;

static CONFIG: OnceCell<Config> = OnceCell::new();
static DATABASE: OnceCell<Pool<Sqlite>> = OnceCell::new();
static CLIENT: OnceCell<Client> = OnceCell::new();
static STORE: OnceCell<Store> = OnceCell::new();

mod ui;

fn main() -> iced::Result {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let config = Config {
        api_url: "http://127.0.0.1:8050/api".to_string(),
        storage_dir: "./".to_string(),
    };

    let db_path = format!("sqlite:{}/app.db", config.storage_dir);

    let pool = iced::executor::Default::new().unwrap().block_on(async move {
        let options = SqliteConnectOptions::from_str(db_path.as_str())
            .unwrap()
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .unwrap();

        sqlx::migrate!("../reax/migrations").run(&pool).await.unwrap();

        pool
    });

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
    Folders(Folders),
    Notes(Notes),
    Note(Note),
}

struct MaviNote {
    page: Page,
}

#[derive(Debug)]
enum Message {
    FoldersMessage(FoldersMessage),
    NotesMessage(NotesMessage),
    NoteMessage(NoteMessage),
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
            (Message::FoldersMessage(FoldersMessage::Navigate(folder_id)), Page::Folders(_)) => {
                let notes = Notes::new(folder_id);
                self.page = Page::Notes(notes.0);
                return notes.1.map(Message::NotesMessage);
            },
            (Message::FoldersMessage(m), Page::Folders(folders)) => folders.update(m),
            (Message::NotesMessage(NotesMessage::BackNavigation), Page::Notes(_)) => {
                let folders = Folders::new();
                self.page = Page::Folders(folders.0);
                return folders.1.map(Message::FoldersMessage);
            },
            (Message::NotesMessage(NotesMessage::Navigate(folder_id, note_id)), Page::Notes(_)) => {
                let note = Note::new(folder_id, note_id);
                self.page = Page::Note(note.0);
                return note.1.map(Message::NoteMessage);
            },
            (Message::NotesMessage(m), Page::Notes(notes)) => notes.update(m),
            (Message::NoteMessage(NoteMessage::BackNavigation(folder_id)), Page::Note(_)) => {
                let notes = Notes::new(folder_id);
                self.page = Page::Notes(notes.0);
                return notes.1.map(Message::NotesMessage);
            },
            (Message::NoteMessage(m), Page::Note(note)) => note.update(m),
            _ => {},
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
            },
            Page::Notes(notes) => {
                let notes = Column::new().push(
                    notes
                        .view()
                        .map(|message| Message::NotesMessage(message)),
                );

                Container::new(notes).into()
            },
            Page::Note(note) => {
                let note = Column::new().push(
                    note
                        .view()
                        .map(|message| Message::NoteMessage(message)),
                );

                Container::new(note).into()
            },
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
