use iced::{button::State, Button, Column, Text, Command, Element};

use base::Error;
use note::models::Folder;

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    FoldersLoaded(Result<Vec<Folder>, Error>)
}

pub struct Folders {
    counter: i32,
    folders: Vec<Folder>,
    increment_button: State,
    decrement_button: State,
}

impl Folders {
    pub fn new() -> (Self, Command<Message>) {
        (
            Self {
                counter: 0,
                folders: Vec::new(),
                increment_button: State::new(),
                decrement_button: State::new(),
            },
            Command::perform(note::folders(crate::store(), crate::client(), crate::config()), Message::FoldersLoaded)
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.counter += 1,
            Message::Decrement => self.counter -= 1,
            Message::FoldersLoaded(Ok(folders)) => self.folders = folders,
            Message::FoldersLoaded(Err(e)) => log::error!("failed to fetch folders, {:?}", e),
        }
    }

    pub fn view(&mut self) -> iced::Element<Message> {
        let folders: Element<_> = self.folders
            .iter_mut()
            .fold(Column::new().spacing(20), |column, folder| {
                column.push(Text::new(folder.name.as_str()))
            })
            .into();

        Column::new()
            .push(folders)
            .push(Text::new(self.counter.to_string()))
            .push(
                Button::new(
                    &mut self.increment_button,
                    Text::new("Increment"),
                )
                .on_press(Message::Increment),
            )
            .push(
                Button::new(
                    &mut self.decrement_button,
                    Text::new("Decrement"),
                )
                .on_press(Message::Decrement),
            )
            .into()
    }
}
