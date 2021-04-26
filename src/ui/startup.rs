use tui::widgets::{Block, Borders};

pub fn startup() {
    let draw =  Block::default()
        .title("Startup")
        .borders(Borders::ALL);
}
