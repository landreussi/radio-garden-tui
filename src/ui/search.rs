use ratatui::{
    prelude::{Alignment, Backend, Rect},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::State;

pub const SEARCH_BAR_HEIGHT: u16 = 3;

pub fn draw(_state: &mut State, frame: &mut Frame<impl Backend>, area: Rect) {
    let search_text: Vec<Line> = vec!["blah".into()];
    let search_bar = Paragraph::new(search_text)
        .block(
            Block::new()
                .title("Search")
                .borders(Borders::ALL)
                .border_type(super::DEFAULT_BORDER_TYPE),
        )
        .alignment(Alignment::Left);
    frame.render_widget(search_bar, area);
}
