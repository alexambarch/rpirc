mod event;
mod ui;
mod app;

use ui::Ui;
use ui::util::UiEvent;
use app::App;
use tokio::sync::mpsc::{Sender, Receiver, channel};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::execute;
use std::io::stdout;

#[tokio::main]
async fn main() {
    let mut app = App::default();
    let (tx, rx): (Sender<UiEvent>, Receiver<UiEvent>) = channel(15);

    // Listen for keyboard events
    let handle = tokio::spawn(event::listen_events(tx));

    // Start UI
    let mut ui = Ui::new();
    ui.listen(rx, &mut app).await;
    execute!(stdout(), LeaveAlternateScreen).unwrap();

    handle.await.unwrap();
    disable_raw_mode().unwrap();
}
