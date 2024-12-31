use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::Paragraph,
    Frame,
};

use super::app::{ActiveSection, App, Mode, Screen};

#[derive(Debug)]
pub struct EditScreen {}

impl EditScreen {
    pub fn draw(app: &mut App, frame: &mut Frame) {
        let global_layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);
        let [main_area, footer_area] = global_layout.areas(frame.area());
        let layout = Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]);
        let [text_area, destinations_area] = layout.areas(main_area);

        frame.render_widget(&mut app.text, text_area);
        frame.render_widget(&mut app.destinations, destinations_area);

        draw_footer(app, footer_area, frame);
    }

    pub fn handle_event(app: &mut App, event: &Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match app.mode {
                    Mode::Normal => match app.active_section {
                        ActiveSection::List => {
                            handle_normal_list_event(app, key);
                        }
                        ActiveSection::Text => {
                            handle_normal_text_event(app, key);
                        }
                    },
                    Mode::Insert => handle_insert_event(app, key),
                }
            }
        }
    }

    pub fn focus_active(app: &mut App) {
        match app.active_section {
            ActiveSection::Text => {
                app.text.focus();
                app.destinations.blur();
                app.destinations.select_none();
            }
            ActiveSection::List => {
                app.text.blur();
                app.destinations.focus();
                app.destinations.select_first();
            }
        }
    }
}

fn draw_footer(app: &mut App, area: Rect, frame: &mut Frame) {
    let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let [left_area, right_area] = layout.areas(area);
    let mode_sigil = match app.mode {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
    };
    let normal_help = vec![
        "exit: q ".into(),
        "fire: f ".into(),
        "nav: h/j/k/l ".into(),
        "mode: i ".into(),
    ];
    let insert_help = vec!["mode: esc ".into()];
    let help = match app.mode {
        Mode::Normal => normal_help,
        Mode::Insert => insert_help,
    };

    let left = Paragraph::new(Line::from(mode_sigil));
    let right = Paragraph::new(Line::from(help)).right_aligned();

    frame.render_widget(left, left_area);
    frame.render_widget(right, right_area);
}

fn handle_normal_list_event(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('j') | KeyCode::Down => app.destinations.select_next(),
        KeyCode::Char('k') | KeyCode::Up => app.destinations.select_previous(),
        KeyCode::Char('h') | KeyCode::Left => {
            app.active_section = ActiveSection::Text;
            EditScreen::focus_active(app);
        }
        KeyCode::Char('l') | KeyCode::Right => {
            app.active_section = ActiveSection::List;
            EditScreen::focus_active(app);
        }
        KeyCode::Enter => app.destinations.toggle_status(),
        KeyCode::Char(' ') => {
            app.destinations.toggle_status();
            app.destinations.select_next();
        }
        KeyCode::Char('f') => app.screen = Screen::Confirm,
        _ => {}
    }
}

fn handle_normal_text_event(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('j') | KeyCode::Down => app.text.down(),
        KeyCode::Char('k') | KeyCode::Up => app.text.up(),
        KeyCode::Char('h') | KeyCode::Left => {
            app.active_section = ActiveSection::Text;
            EditScreen::focus_active(app);
        }
        KeyCode::Char('l') | KeyCode::Right => {
            app.active_section = ActiveSection::List;
            EditScreen::focus_active(app);
        }
        KeyCode::Char('i') => {
            if app.active_section == ActiveSection::Text {
                app.mode = Mode::Insert;
            }
        }
        KeyCode::Char('f') => app.screen = Screen::Confirm,
        _ => {}
    }
}

fn handle_insert_event(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => app.mode = Mode::Normal,
        _ => {
            app.text.input(key);
        }
    }
}
