use super::{BORDER_ACTIVE_STYLE, BORDER_INACTIVE_STYLE, HIGHLIGHT_STYLE};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget,
    },
};

#[derive(Debug)]
pub(super) struct DestinationList {
    pub items: Vec<DestinationItem>,
    pub state: ListState,
    pub border_style: Style,
}

impl Default for DestinationList {
    fn default() -> Self {
        Self {
            items: vec![
                DestinationItem::new("Bluesky"),
                DestinationItem::new("LinkedIn"),
                DestinationItem::new("Turso"),
            ],
            state: ListState::default(),
            border_style: Style::default(),
        }
    }
}

impl DestinationList {
    pub fn select_none(&mut self) {
        self.state.select(None);
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn toggle_status(&mut self) {
        use DestinationStatus::*;

        if let Some(i) = self.state.selected() {
            self.items[i].status = match self.items[i].status {
                Active => Inactive,
                Inactive => Active,
            };
        }
    }

    pub fn focus(&mut self) {
        self.border_style = BORDER_ACTIVE_STYLE;
    }

    pub fn blur(&mut self) {
        self.border_style = BORDER_INACTIVE_STYLE;
    }
}

impl Widget for &mut DestinationList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title("Destinations")
            .borders(Borders::ALL)
            .border_style(self.border_style);
        let items = self.items.iter().map(|item| ListItem::from(item));
        let list = List::new(items)
            .block(block)
            .highlight_style(HIGHLIGHT_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

#[derive(Debug, Clone)]
pub(super) struct DestinationItem {
    pub text: String,
    pub status: DestinationStatus,
}

impl DestinationItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            status: DestinationStatus::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(super) enum DestinationStatus {
    #[default]
    Inactive,
    Active,
}

impl From<&DestinationItem> for ListItem<'_> {
    fn from(value: &DestinationItem) -> Self {
        use DestinationStatus::*;

        let line = match value.status {
            Inactive => Line::styled(
                format!(" ☐ {}", value.text),
                Style::default().fg(Color::Gray),
            ),
            Active => Line::styled(
                format!(" ✓ {}", value.text),
                Style::default().fg(Color::LightGreen),
            ),
        };
        ListItem::new(line)
    }
}
