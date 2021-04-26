mod event;
mod ui;

use std::io::stdout;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode}
};
use anyhow::Result;
use tui::Terminal;
use tui::widgets::{Block, Borders, ListItem, List};
use tui::style::{Style, Modifier, Color};
use tui::backend::CrosstermBackend;
use event::terminal;
use tokio::task;
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<()> {
    // TODO User config setup

    // Most recent servers connected to
    let qc_servers: Vec<&str> = vec![];


    // Start terminal
    let stdout = stdout();
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.draw(|f| {
        let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
        let draw = List::new(items)
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        let size = f.size();
        f.render_widget(draw, size);
    });

    // Spawn input handler
    task::spawn(terminal::receive_terminal_events());

    thread::sleep(time::Duration::from_secs(10));

    disable_raw_mode()?;
    Ok(())
}
