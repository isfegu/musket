use futures::StreamExt;
use ratatui::{
    crossterm::event::{Event, EventStream},
    DefaultTerminal, Frame,
};
use std::time::Duration;

use crate::MusketError;

use super::{
    confirm_screen::ConfirmScreen, destination_list::DestinationStatus, textarea::Textbox,
};
use super::{destination_list::DestinationList, edit_screen::EditScreen};

#[derive(Debug, Default)]
pub struct App<'text> {
    pub(crate) should_quit: bool,
    pub(crate) should_publish: bool,
    pub(crate) screen: Screen,
    pub(crate) active_section: ActiveSection,
    pub(crate) mode: Mode,
    pub(crate) text: Textbox<'text>,
    pub(crate) destinations: DestinationList,
}

impl App<'_> {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<Option<Pack>, MusketError> {
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        // Set initial state.
        EditScreen::focus_active(&mut self);

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| {
                    self.draw(frame);
                })?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }

        terminal.clear()?;

        if self.should_publish {
            let pack = self.into_pack();

            if pack.message.is_none() {
                // We can't publish nothing!
                return Err(MusketError::Cli {
                    message: "You surely want something to publish, don't you?".into(),
                });
            }

            if pack.destinations.is_empty() {
                // We can't publish nowhere!
                return Err(MusketError::Cli {
                    message: "You surely want to publish somewhere, don't you?".into(),
                });
            }

            Ok(Some(pack))
        } else {
            Ok(None)
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        match self.screen {
            Screen::Edit => EditScreen::draw(self, frame),
            Screen::Confirm => ConfirmScreen::draw(self, frame),
            // Screen::Publish => todo!(),
        }
    }

    fn handle_event(&mut self, event: &Event) {
        match self.screen {
            Screen::Edit => EditScreen::handle_event(self, event),
            Screen::Confirm => ConfirmScreen::handle_event(self, event),
            // Screen::Publish => todo!(),
        }
    }

    pub(crate) fn into_pack(&mut self) -> Pack {
        let destinations = self
            .destinations
            .items
            .clone()
            .iter()
            .filter(|item| item.status == DestinationStatus::Active)
            .map(|item| item.text.clone())
            .collect::<Vec<String>>();
        let lines = self
            .text
            .lines()
            .into_iter()
            .map(|line| line)
            .collect::<Vec<_>>();

        // WARN: tui-textarea always returns at least one line.
        // If the first line is empty, we understand there is no message.
        let message = if lines[0].trim() == "" {
            None
        } else {
            Some(lines)
        };

        Pack {
            message,
            destinations,
        }
    }
}

/// A packed representation of data to send to each destination.
#[derive(Debug, Clone)]
pub struct Pack {
    pub message: Option<Vec<String>>,
    pub destinations: Vec<String>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ActiveSection {
    #[default]
    Text,
    List,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Edit,
    Confirm,
    // Publish,
}
