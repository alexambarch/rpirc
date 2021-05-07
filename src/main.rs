mod event;
mod ui;
mod app;

use ui::Ui;
use ui::util::UiEvent;
use app::App;
use tokio::sync::mpsc::{Sender, Receiver, channel};

#[tokio::main]
async fn main() {
    let mut app = App::default();
    let (tx, mut rx): (Sender<UiEvent>, Receiver<UiEvent>) = channel(15);

    // Listen for keyboard events
    tokio::spawn(event::listen_events(tx));

    // Start UI
    let mut ui = Ui::new();
    ui.listen(rx, &mut app);
}
