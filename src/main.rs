mod event;
mod ui;
mod handlers;

use crate::handlers::tui::*;

#[tokio::main]
async fn main() {

    // Run ui
    let mut ui = Ui::new();
    ui.listen();
}
