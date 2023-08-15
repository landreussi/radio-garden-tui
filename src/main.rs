use std::io::{stdout, Result, Stdout};

use api::{Place, RadioGardenApi};
use crossterm::{
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use url::Url;

trait Split {
    fn split_horizontally(&self, n: u16) -> Vec<Rect>;
}

impl Split for Rect {
    fn split_horizontally(&self, n: u16) -> Vec<Rect> {
        let width = self.width / n;
        let height = self.height;
        let mut x = self.x;
        let y = self.y;
        let mut rectangles = Vec::with_capacity(n.into());

        for _ in 0..n {
            let rect = Rect::new(x, y, width, height);
            rectangles.push(rect);
            x += width;
        }

        rectangles
    }
}

// This should be an external function as it could be used for every `StatefulWidget` that moves.
fn r#move(state: &mut ListState, up: bool, mut max_value: usize) {
    let mut selected = state.selected();
    {
        let selected = selected.get_or_insert(0);
        if up {
            if selected > &mut 0 {
                *selected -= 1;
            }
        } else if selected < &mut max_value {
            *selected += 1;
        }
    }
    state.select(selected);
}

#[derive(Default)]
#[repr(u8)]
enum Focus {
    #[default]
    Countries = 1,
    Cities = 2,
}

impl Focus {
    const MAX: u8 = 2;
    fn from_u8(num: u8) -> Self {
        match num {
            1 => Self::Countries,
            2 => Self::Cities,
            _ => unreachable!(),
        }
    }
    fn to_u8(&self) -> u8 {
        match self {
            Self::Countries => 1,
            Self::Cities => 2,
        }
    }
}

#[derive(Default)]
struct State {
    countries: ListState,
    cities: ListState,
    focus: Focus,
}

impl State {
    fn move_focus(&mut self, right: bool) {
        let mut focus = self.focus.to_u8();
        if right {
            if focus == Focus::MAX {
                focus = 1;
            } else {
                focus += 1;
            }
        } else if focus == 1 {
            focus = Focus::MAX;
        } else {
            focus -= 1;
        }

        self.focus = Focus::from_u8(focus);
    }
    fn focused(&mut self) -> &mut ListState {
        match self.focus {
            Focus::Cities => &mut self.cities,
            Focus::Countries => &mut self.countries,
        }
    }
}

impl State {
    fn new() -> Self {
        let mut state = Self::default();
        state.countries.select(Some(0));
        state.cities.select(Some(0));

        state
    }
}

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
    async fn fill_places(&mut self) {
        let places = self.api.list_places().await.unwrap();

        self.places = places;
    }
    fn draw(&mut self) -> Result<()> {
        self.terminal
            .draw(|f| Self::draw_entrypoint(&mut self.state, f, &self.places))?;

        Ok(())
    }
    fn draw_entrypoint(state: &mut State, f: &mut Frame<impl Backend>, places: &[Place]) {
        let rects = f.size().split_horizontally(2);
        let countries: Vec<_> = places
            .iter()
            .map(|p| ListItem::new(p.country.as_str()))
            .collect();
        let countries_list = List::new(countries)
            .block(Block::default().title("Countries").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black));
        f.render_stateful_widget(countries_list, rects[0], &mut state.countries);

        let cities: Vec<_> = places
            .iter()
            .map(|p| ListItem::new(p.city.as_str()))
            .collect();
        let cities_list = List::new(cities)
            .block(Block::default().title("Cities").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black));
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

    fn key_event(&mut self, key: KeyEvent) {
        const ESC: KeyEvent = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        const UP: KeyEvent = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        const DOWN: KeyEvent = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        const LEFT: KeyEvent = KeyEvent::new(KeyCode::Left, KeyModifiers::NONE);
        const RIGHT: KeyEvent = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);

        // TODO: transform this in a match.
        if key == ESC {
            self.close()
        } else if key == UP {
            self.move_up()
        } else if key == DOWN {
            self.move_down()
        } else if key == RIGHT {
            self.move_right()
        } else if key == LEFT {
            self.move_left()
        }

        self.draw().unwrap();
    }

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

    app.fill_places().await;
    app.draw()?;
    app.event_loop();

    Ok(())
}
