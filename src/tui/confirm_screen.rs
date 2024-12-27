use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use super::app::{App, Screen};

#[derive(Debug)]
pub struct ConfirmScreen {}

impl ConfirmScreen {
    pub fn draw(app: &mut App, frame: &mut Frame) {
        let [main_area, footer_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());
        let [_, dest_area, text_area, _] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(main_area);
        let [_, key_text_area, value_text_area] = Layout::horizontal([
            Constraint::Length(1),
            Constraint::Length(16),
            Constraint::Fill(1),
        ])
        .areas(text_area);
        let [_, key_dest_area, value_dest_area] = Layout::horizontal([
            Constraint::Length(1),
            Constraint::Length(16),
            Constraint::Fill(1),
        ])
        .areas(dest_area);

        let confirmation_block =
            Block::bordered().title_top(Line::from(" Ready to publish? ").centered());
        frame.render_widget(confirmation_block, main_area);

        let pack = app.into_pack();
        let destinations = pack.destinations;
        let content = if let Some(message) = pack.message {
            message
                .into_iter()
                .map(|line| Line::from(line))
                .collect::<Vec<Line>>()
        } else {
            vec![Line::from("<No content>")]
        };
        let message = Paragraph::new(content);
        let key_style = Style::new().fg(Color::Yellow);

        frame.render_widget(Span::styled(" Destinations", key_style), key_dest_area);
        frame.render_widget(destinations.join(", "), value_dest_area);
        frame.render_widget(Span::styled(" Message", key_style), key_text_area);
        frame.render_widget(message, value_text_area);

        draw_footer(footer_area, frame);
    }

    pub fn handle_event(app: &mut App, event: &Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Char('n') => app.screen = Screen::Edit,
                    KeyCode::Char('y') => {
                        // TODO: Add a Publish screen displaying progress
                        // app.screen = Screen::Publish
                        //
                        // For now we just flag we want to publish and quit the TUI application.
                        app.should_publish = true;
                        app.should_quit = true;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn draw_footer(area: Rect, frame: &mut Frame) {
    let help = vec!["exit: q ".into(), "cancel: n ".into(), "confirm: y ".into()];
    let line = Paragraph::new(Line::from(help)).right_aligned();

    frame.render_widget(line, area);
}
