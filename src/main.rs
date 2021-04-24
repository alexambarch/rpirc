mod event;

use std::io::stdout;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode}
};
use anyhow::Result;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use event::terminal;

#[tokio::main]
async fn main() -> Result<()> {
    // TODO User config setup

    // TODO Call draw loop

    // TODO Call event handler loop

    // Start terminal
    let mut stdout = stdout();
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal::receive_terminal_events().await;
    disable_raw_mode()?;

    Ok(())
}
