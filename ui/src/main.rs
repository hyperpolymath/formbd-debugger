// SPDX-License-Identifier: AGPL-3.0-or-later
//! FormDB Debugger Terminal UI
//!
//! A Ratatui-based terminal interface for database debugging,
//! recovery plan generation, and proof verification.

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

mod app;
mod widgets;

/// FormDB Debugger - Proof-carrying database recovery
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Database connection string
    #[arg(short, long)]
    connect: Option<String>,

    /// Database type (postgres, formdb, sqlite)
    #[arg(short = 't', long, default_value = "formdb")]
    db_type: String,

    /// Run in non-interactive mode
    #[arg(long)]
    batch: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.batch {
        // Batch mode - just run commands
        println!("FormDB Debugger v0.1.0 (batch mode)");
        if let Some(conn) = &args.connect {
            println!("Connecting to: {}", conn);
        }
        return Ok(());
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal, &args);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    _args: &Args,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(f.area());

            // Header
            let header = Paragraph::new("FormDB Debugger v0.1.0")
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL).title("Header"));
            f.render_widget(header, chunks[0]);

            // Main content
            let content = Paragraph::new("Press 'q' to quit\n\nNot connected to any database.")
                .block(Block::default().borders(Borders::ALL).title("Main"));
            f.render_widget(content, chunks[1]);

            // Footer
            let footer = Paragraph::new("help | connect | schema | timeline | diagnose | recover")
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::default().borders(Borders::ALL).title("Commands"));
            f.render_widget(footer, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
