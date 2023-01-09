extern crate core;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::task;

use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use tui::{
    backend::TermionBackend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::widgets::{Cell, Row, Table, TableState, Wrap};
use futures::prelude::*;
use ssdp_client::URN;
use ssdp_client::SearchTarget;
use reqwest;

struct App {
    state: TableState,
    items: Vec<Vec<String>>,
}


impl App {

    fn new(devices:Vec<Vec<String>>) -> App {

        App {
            state: TableState::default(),
            items: devices,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn enter(&mut self){
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}


async fn scan_network() -> Vec<Vec<String>> {
    let search_target = SearchTarget::RootDevice;
    println!("Loading...");
    let mut responses = ssdp_client::search(&search_target, Duration::from_secs(3), 5).await.unwrap();
    let mut devices:Vec<Vec<String>>= Vec::new();
    while let Some(response) = responses.next().await {
        let response = response.unwrap();
        let server:String = response.server().to_owned();
        let location:String = response.location().to_owned();

        let mut row: Vec<String> = vec![
            server,
            location

        ];
        devices.push( row);
    }
    return devices
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let devices_task = tokio::task::spawn(scan_network());


    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(devices_task.await.unwrap());
    let res = run_app(&mut terminal, app);

    // // restore terminal
    disable_raw_mode()?;
    execute!(
         terminal.backend_mut(),
         LeaveAlternateScreen,
         DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
         println!("{:?}", err)
    }

    Ok(())
}



fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {


    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Enter =>app.enter(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default();
    let header_cells = ["Server", "Location"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;

        let cells = item.iter().map(|c| Cell::from(&**c));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(t, rects[0], &mut app.state);
}