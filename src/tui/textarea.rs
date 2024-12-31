use super::{BORDER_ACTIVE_STYLE, BORDER_INACTIVE_STYLE};
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyEvent,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Widget},
};
use tui_textarea::{CursorMove, Input, TextArea};

#[derive(Debug, Default)]
pub struct Textbox<'text> {
    textarea: TextArea<'text>,
    border_style: Style,
}

impl Textbox<'_> {
    pub fn focus(&mut self) {
        self.border_style = BORDER_ACTIVE_STYLE;
    }

    pub fn blur(&mut self) {
        self.border_style = BORDER_INACTIVE_STYLE;
    }

    pub fn lines(&self) -> Vec<String> {
        self.textarea.lines().into()
    }

    pub fn input(&mut self, key: &KeyEvent) {
        self.textarea.input(Input::from(key.clone()));
    }

    pub fn down(&mut self) {
        self.textarea.move_cursor(CursorMove::Down);
    }

    pub fn up(&mut self) {
        self.textarea.move_cursor(CursorMove::Up);
    }
}

impl Widget for &mut Textbox<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(self.border_style)
                .title("Text to send"),
        );

        self.textarea.render(area, buf);
    }
}
