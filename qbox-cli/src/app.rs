use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use qbox_core::broker::Level1;
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

pub struct App {
    state: TableState,
    level1: Vec<Level1>,
}

impl App {
    fn new() -> App {
        App {
            state: TableState::default(),
            level1: vec![],
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.level1.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.level1.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub(crate) fn run_app() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let mut last_tick = Instant::now();
    // Input
    loop {
        if let Some(levels) = qbox_core::get_all_level1() {
            app.level1 = levels;
        }
        let tick_rate = Duration::from_millis(200);
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        terminal.draw(|f| ui(f, &mut app))?;
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = [
        "名称",
        "代码",
        "均价",
        "成交量",
        "最新价",
        "开盘价",
        "最高价",
        "最低价",
        "收盘价",
        "score",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(0);
    let rows = app.level1.iter().map(|item| {
        let name = if let Some(instr) = qbox_core::get_instrument(&item.security_id) {
            instr.symbol
        } else {
            item.security_id.clone()
        };
        let cells = vec![
            Cell::from(name),
            Cell::from(item.security_id.clone()),
            Cell::from(item.average.to_string()),
            Cell::from(item.volume.to_string()),
            Cell::from(item.last.to_string()),
            Cell::from(item.open.to_string()),
            Cell::from(item.high.to_string()),
            Cell::from(item.low.to_string()),
            Cell::from(item.close.to_string()),
            Cell::from(item.score.to_string()),
        ];
        Row::new(cells).bottom_margin(0)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("期货市场"))
        .highlight_style(selected_style)
        //.highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ]);
    f.render_stateful_widget(t, rects[0], &mut app.state);
}
