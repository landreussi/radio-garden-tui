use std::io::{stdout, Result};

pub mod ext;
pub mod state;
pub mod ui;

use std::io::Write;

use api::RadioGardenApi;
use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ext::Split;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    prelude::Rect,
    Frame, Terminal,
};
use state::{Focus, State};
use url::Url;

struct App<B: Backend> {
    api: RadioGardenApi,
    terminal: Terminal<B>,
    state: State,
}

impl<B> App<B>
where
    B: Backend + Write,
{
    fn new(api: RadioGardenApi, terminal: Terminal<B>) -> Self {
        Self {
            api,
            terminal,
            state: State::new(),
        }
    }
    async fn search_places(&mut self) {
        let places = self.api.list_places().await.unwrap();

        self.state.places = places;
    }
    fn draw(&mut self) -> Result<()> {
        self.terminal
            .draw(|f| Self::draw_entrypoint(&mut self.state, f))?;

        Ok(())
    }
    fn draw_entrypoint(state: &mut State, frame: &mut Frame<B>) {
        let mut terminal_area = frame.size();

        let search_bar_area = Rect::new(0, 0, terminal_area.width, ui::search::SEARCH_BAR_HEIGHT);
        ui::search::draw(state, frame, search_bar_area);
        terminal_area.y = ui::search::SEARCH_BAR_HEIGHT;
        terminal_area.height -= ui::search::SEARCH_BAR_HEIGHT;

        let rects = terminal_area.split_vertically(2);

        ui::countries::draw(state, frame, rects[0]);
        ui::cities::draw(state, frame, rects[1]);
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
        if self.state.focus == Focus::Search {
            match code {
                KeyCode::Down => self.state.move_down(),
                KeyCode::Char(c) => self.state.search.push(c),
                KeyCode::Backspace => {
                    let _ = self.state.search.pop();
                }
                KeyCode::Esc => self.close(),
                _ => {}
            }
        } else {
            match code {
                KeyCode::Esc => self.close(),
                KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => self.state.move_up(),
                KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => self.state.move_down(),
                KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => self.state.move_left(),
                KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Tab => {
                    self.state.move_right()
                }
                KeyCode::Char('/') => self.state.move_to_search(),
                _ => {
                    // tracing::info!("");
                }
            }
        }

        self.draw().unwrap();
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

    app.search_places().await;
    app.draw()?;
    app.event_loop();

    Ok(())
}
