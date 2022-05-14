use base::Error;
use iced::{button::State as ButtonState, text_input::State as TextInputState, Command, Element, Text, Column, Button, TextInput};
use note::models::Note as NoteModel;

#[derive(Clone, Debug)]
pub enum Message {
    BackNavigation(i32),
    NoteLoaded(Result<Option<NoteModel>, Error>),
    NoteUpdated(String),
}

pub struct Note {
    folder_id: i32,
    note: Option<NoteModel>,
    note_input: TextInputState,
    back_button: ButtonState,
}

impl Note {
    pub fn new(folder_id: i32, note_id: i32) -> (Self, Command<Message>) {
        (
            Self {
                folder_id,
                note: None,
                note_input: TextInputState::new(),
                back_button: ButtonState::new(),
            },
            Command::perform(note::note(crate::store(), crate::client(), crate::config(), note_id), Message::NoteLoaded)
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::BackNavigation(_) => log::error!("BackNavigation should be handled by parent"),
            Message::NoteLoaded(Ok(note)) => self.note = note,
            Message::NoteLoaded(Err(e)) => log::error!("failed to fetch note, {:?}", e),
            Message::NoteUpdated(text) => self.note.as_mut().unwrap().text = text,
        }
    }

    pub fn view(&mut self) -> iced::Element<Message> {
        let mut column = Column::new();
        if let Some(note) = &self.note {
            column = column
                .push(Text::new(note.title.as_str()))
                .push(TextInput::new(&mut self.note_input, "Note", note.text.as_str(), Message::NoteUpdated))
        } else {
            column = column
                .push(Text::new("Note could not be found"))
        };

        column = column.push(Button::new(&mut self.back_button, Text::new("Back")).on_press(Message::BackNavigation(self.folder_id)));

        column.into()
    }
}
