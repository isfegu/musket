mod app;
mod confirm_screen;
mod destination_list;
mod edit_screen;
mod textarea;

// use color_eyre::Result;
use ratatui::{
    style::{Color, Style},
    TerminalOptions, Viewport,
};

use app::{App, Pack};

use crate::MusketError;

// Palette
const BORDER_ACTIVE_STYLE: Style = Style::new().fg(Color::Yellow);
const BORDER_INACTIVE_STYLE: Style = Style::new().fg(Color::Gray);
const HIGHLIGHT_STYLE: Style = Style::new().fg(Color::Yellow).bg(Color::Black);

pub async fn main() -> Result<Option<Pack>, MusketError> {
    // color_eyre::install()?;
    let terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(10),
    });
    let app = App::default();
    let app_result = app.run(terminal).await;
    ratatui::restore();

    app_result
}
