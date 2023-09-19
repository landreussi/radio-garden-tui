use ratatui::{
    prelude::{Backend, Rect},
    style::Style,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::state::State;

pub fn draw(state: &mut State, frame: &mut Frame<impl Backend>, area: Rect) {
    let countries: Vec<_> = state
        .places
        .iter()
        .map(|p| ListItem::new(p.country.as_str()))
        .collect();
    let countries_bg = state.focus.color_from(&crate::state::Focus::Countries);
    let countries_list = List::new(countries)
        .block(
            Block::default()
                .title("Countries")
                .borders(Borders::ALL)
                .border_type(super::DEFAULT_BORDER_TYPE),
        )
        .style(Style::default().fg(countries_bg))
        .highlight_style(
            Style::default()
                .bg(countries_bg)
                .fg(super::DEFAULT_FG_COLOR),
        );
    frame.render_stateful_widget(countries_list, area, &mut state.countries);
}
