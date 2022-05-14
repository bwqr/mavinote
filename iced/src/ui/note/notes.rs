use base::Error;
use iced::{button::State, Command, Element, Column, Button, Text};
use note::models::Note;

#[derive(Clone, Debug)]
pub enum Message {
    BackNavigation,
    Navigate(i32, i32),
    NotesLoaded(Result<Vec<Note>, Error>)
}

pub struct Notes {
    notes: Vec<(Note, State)>,
    back_button: State,
}

impl Notes {
    pub fn new(folder_id: i32) -> (Self, Command<Message>) {
        (
            Self {
                notes: Vec::new(),
                back_button: State::new(),
            },
            Command::perform(note::note_summaries(crate::store(), crate::client(), crate::config(), folder_id), Message::NotesLoaded)
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::BackNavigation => log::error!("BackNavigation should be handled by parent"),
            Message::Navigate(..) => log::error!("Navigate should be handled by parent"),
            Message::NotesLoaded(Ok(notes)) => self.notes = notes.into_iter().map(|note| (note, State::new())).collect(),
            Message::NotesLoaded(Err(e)) => log::error!("failed to fetch folders, {:?}", e),
        }
    }

    pub fn view(&mut self) -> iced::Element<Message> {
        let notes: Element<_> = self.notes
            .iter_mut()
            .fold(Column::new().spacing(20), |column, note| {
                column.push(
                    Button::new(&mut note.1, Text::new(note.0.title.as_str()))
                        .on_press(Message::Navigate(note.0.folder_id, note.0.id))
                )
            })
            .into();

        Column::new()
            .push(notes)
            .push(Button::new(&mut self.back_button, Text::new("Back")).on_press(Message::BackNavigation))
            .into()
    }
}
