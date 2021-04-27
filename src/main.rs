mod event;
mod ui;
use crate::ui::*;

#[tokio::main]
async fn main() {

    // Run ui
    let mut ui = Ui::new();
    ui.listen().await;
}
