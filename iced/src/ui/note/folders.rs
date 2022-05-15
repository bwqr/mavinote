use iced::{button::State, Button, Column, Command, Element, Text};

use base::{Error, Store, Config};
use note::models::Folder;
use reqwest::Client;

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(i32),
    FoldersLoaded(Result<Vec<Folder>, Error>),
}

pub struct Folders {
    folders: Vec<(Folder, State)>,
}

impl Folders {
    pub fn new() -> (Self, Command<Message>) {
        (
            Self {
                folders: Vec::new(),
            },
            Command::perform(
                note::folders(
                    runtime::get::<Store>().unwrap(),
                    runtime::get::<Client>().unwrap(),
                    runtime::get::<Config>().unwrap(),
                ),
                Message::FoldersLoaded,
            ),
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Navigate(_) => log::error!("this message must be catched by parent"),
            Message::FoldersLoaded(Ok(folders)) => {
                self.folders = folders
                    .into_iter()
                    .map(|folder| (folder, State::new()))
                    .collect()
            }
            Message::FoldersLoaded(Err(e)) => log::error!("failed to fetch folders, {:?}", e),
        }
    }

    pub fn view(&mut self) -> iced::Element<Message> {
        let folders: Element<_> = self
            .folders
            .iter_mut()
            .fold(Column::new().spacing(20), |column, folder| {
                column.push(
                    Button::new(&mut folder.1, Text::new(folder.0.name.as_str()))
                        .on_press(Message::Navigate(folder.0.id)),
                )
            })
            .into();

        Column::new().push(folders).into()
    }
}
