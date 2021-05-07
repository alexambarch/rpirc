use anyhow::Result;
use crossterm::event::{EventStream, Event, Event::Key,
                       KeyEvent, KeyCode, KeyModifiers};
use tokio::sync::mpsc::Sender;
use futures::{future::FutureExt, StreamExt};
use crate::ui::util::*;


/* listen_events()
 *
 * Listen for keyboard events and send the appropriate message to the UI
 * */

pub async fn listen_events(tx: Sender<UiEvent>) -> Result<()> {
    let mut reader = EventStream::new();

    while let Some(event) = reader.next().fuse().await {
        match event.unwrap() {
            // Send character keypresses to text input handler
            Key(KeyEvent{code: KeyCode::Char(ch),
                         modifiers: KeyModifiers::NONE}) => {
                send_ui_event(UiEvent::Buffer(ch), &tx).await;
            }

            // Backspace deletes a character from the buffer.
            Key(KeyEvent{code: KeyCode::Backspace,
                                modifiers: KeyModifiers::NONE}) => {
                send_ui_event(UiEvent::Del, &tx).await;
            }

            // Enter key executes whatever is inside of the input buffer.
            Key(KeyEvent{code: KeyCode::Enter,
                         modifiers: KeyModifiers::NONE}) => {
                send_ui_event(UiEvent::Execute, &tx).await;
            }

            // Arrow keys move cursor inside of input
            Key(KeyEvent{code: KeyCode::Left,
                         modifiers: KeyModifiers::NONE}) => {
                send_ui_event(UiEvent::Left, &tx).await;
            }

            Key(KeyEvent{code: KeyCode::Right,
                         modifiers: KeyModifiers::NONE}) => {
                send_ui_event(UiEvent::Right, &tx).await;
            }

            // Keyboard Interrupt
            Event::Key(KeyEvent{code: KeyCode::Char('c'),
                                modifiers: KeyModifiers::CONTROL}) => {
                send_ui_event(UiEvent::Terminate, &tx).await;
                break;
            }

            // Some other case I'm sure I'm forgetting
            _ => {}
        }
    }

    Ok(())
}

async fn send_ui_event (event: UiEvent, tx: &Sender<UiEvent>) {
    if let Err(e) = tx.send(event).await {
        eprintln!("{}", e);
    }
}
