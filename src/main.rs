use std::io::{stdout, Result, Stdout};

pub mod ext;
use ext::*;
pub mod state;
use api::{Place, RadioGardenApi};
use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    prelude::{Alignment, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem, Wrap},
    Frame, Terminal,
};
use state::*;
use url::Url;

struct App {
    api: RadioGardenApi,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    state: State,
    places: Vec<Place>,
}

impl App {
    fn new(api: RadioGardenApi, terminal: Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self {
            api,
            terminal,
            state: State::new(),
            places: vec![],
        }
    }
    async fn search_places(&mut self, _query: &str) {
        let places = self.api.list_places().await.unwrap();

        self.places = places;
    }
    fn draw(&mut self) -> Result<()> {
        self.terminal
            .draw(|f| Self::draw_entrypoint(&mut self.state, f, &self.places))?;

        Ok(())
    }
    fn draw_entrypoint(state: &mut State, f: &mut Frame<impl Backend>, places: &[Place]) {
        // TODO: break this in functions to draw every component.
        use ratatui::{text::Line, widgets::Paragraph};
        let search_text: Vec<Line> = vec!["blah".into()];
        let search_bar = Paragraph::new(search_text)
            .block(
                Block::new()
                    .title("Search")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Left);
        let search_bar_area = Rect::new(0, 0, f.size().width, 3);
        f.render_widget(search_bar, search_bar_area);

        let mut rect = f.size();
        rect.y = 3;
        rect.height -= 3;
        let rects = rect.split_horizontally(2);
        let countries: Vec<_> = places
            .iter()
            .map(|p| ListItem::new(p.country.as_str()))
            .collect();
        let countries_fg = state.focus.color_from(&Focus::Countries);
        let countries_list = List::new(countries)
            .block(
                Block::default()
                    .title("Countries")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(countries_fg))
            .highlight_style(Style::default().bg(countries_fg).fg(Color::Black));
        f.render_stateful_widget(countries_list, rects[0], &mut state.countries);

        let cities: Vec<_> = places
            .iter()
            .map(|p| ListItem::new(p.city.as_str()))
            .collect();
        let cities_fg = state.focus.color_from(&Focus::Cities);
        let cities_list = List::new(cities)
            .block(
                Block::default()
                    .title("Cities")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(cities_fg))
            .highlight_style(Style::default().bg(cities_fg).fg(Color::Black));
        f.render_stateful_widget(cities_list, rects[1], &mut state.cities);
    }

    // TODO: this should be async in the future.
    fn event_loop(&mut self) {
        loop {
            if let Event::Key(key) = read().unwrap() {
                self.key_event(key);
            }
        }
    }

    fn key_event(&mut self, KeyEvent { code, .. }: KeyEvent) {
        match code {
            KeyCode::Esc => self.close(),
            KeyCode::Up | KeyCode::Char('k') => self.move_up(),
            KeyCode::Down | KeyCode::Char('j') => self.move_down(),
            KeyCode::Left | KeyCode::Char('h') => self.move_left(),
            KeyCode::Right | KeyCode::Char('l') => self.move_right(),
            _ => unreachable!("unreachable yet, will tracing::info when we do loggings"),
        }

        self.draw().unwrap();
    }

    // TODO: this should be moved to state mod, including the list of places.
    fn move_up(&mut self) {
        let focused = self.state.focused();
        r#move(focused, true, self.places.len())
    }
    fn move_down(&mut self) {
        let focused = self.state.focused();
        r#move(focused, false, self.places.len())
    }
    fn move_right(&mut self) {
        self.state.move_focus(true)
    }
    fn move_left(&mut self) {
        self.state.move_focus(false)
    }

    // TODO: treat right those unwraps.
    fn close(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        self.terminal.show_cursor().unwrap();
        std::process::exit(0);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: Check if we need a config file or env vars.
    let url = Url::parse("https://radio.garden/api/ara/content/").unwrap();
    let api = RadioGardenApi::new(url);
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let mut app = App::new(api, terminal);

    app.search_places("").await;
    app.draw()?;
    app.event_loop();

    Ok(())
}
