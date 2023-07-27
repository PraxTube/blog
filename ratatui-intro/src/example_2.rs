use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{self, prelude::*, widgets::*};

pub fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.clear()?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(size);

    let sub_chunks_left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);
    let sub_chunks_right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(chunks[1]);

    let first_block = Block::default()
        .style(Style::default().bg(Color::Green))
        .title(ratatui::widgets::block::Title::from("1 Title"));
    f.render_widget(first_block, sub_chunks_left[0]);
    let second_block = Block::default()
        .style(Style::default().bg(Color::Yellow))
        .title(ratatui::widgets::block::Title::from("2 Title"));
    f.render_widget(second_block, sub_chunks_left[1]);
    let third_block = Block::default()
        .style(Style::default().bg(Color::Blue))
        .title(ratatui::widgets::block::Title::from("3 Title"));
    f.render_widget(third_block, sub_chunks_right[0]);
    let fourth_block = Block::default()
        .style(Style::default().bg(Color::Red))
        .title(ratatui::widgets::block::Title::from("4 Title"));
    f.render_widget(fourth_block, sub_chunks_right[1]);
}
