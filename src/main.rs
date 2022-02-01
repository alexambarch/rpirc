mod app;
mod event;
mod ui;

use app::App;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use std::io::stdout;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use ui::util::UiEvent;
use ui::Ui;

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
