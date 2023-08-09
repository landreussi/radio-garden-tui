use std::io::{stdout, Result, Stdout};

use api::RadioGardenApi;
use crossterm::{
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: Check if we need a config file or env vars.
    let url = Url::parse("https://radio.garden/api/ara/content/").unwrap();
    let api = RadioGardenApi::new(url);
    let places = api.list_places().await.unwrap().data.list;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let items: Vec<_> = places
            .iter()
            .map(|p| ListItem::new(format!("{} - {}", p.country, p.title)))
            .collect();
        let list = List::new(items)
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        f.render_widget(list, size);
    })?;

    event_loop(&mut terminal);

    Ok(())
}

// TODO: this should be async in the future.
fn event_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    loop {
        let evt = read().unwrap();
        match evt {
            Event::Key(key) => key_event(key, terminal),
            _ => {}
        }
    }
}

fn key_event(key: KeyEvent, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    const ESC: KeyEvent = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    if key == ESC {
        close(terminal);
    }
}

// TODO: treat right those unwraps.
fn close(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
    std::process::exit(0);
}
