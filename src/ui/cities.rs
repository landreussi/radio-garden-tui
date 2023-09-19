use ratatui::{
    prelude::{Backend, Rect},
    style::Style,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::state::State;

pub fn draw(state: &mut State, frame: &mut Frame<impl Backend>, area: Rect) {
    let cities: Vec<_> = state
        .places
        .iter()
        .map(|p| ListItem::new(p.city.as_str()))
        .collect();
    let cities_bg = state.focus.color_from(&crate::state::Focus::Cities);
    let cities_list = List::new(cities)
        .block(
            Block::default()
                .title("Countries")
                .borders(Borders::ALL)
                .border_type(super::DEFAULT_BORDER_TYPE),
        )
        .style(Style::default().fg(cities_bg))
        .highlight_style(Style::default().bg(cities_bg).fg(super::DEFAULT_FG_COLOR));
    frame.render_stateful_widget(cities_list, area, &mut state.cities);
}
