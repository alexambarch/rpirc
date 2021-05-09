use crossterm::event::{EventStream, Event::Key,
                       KeyEvent, KeyCode, KeyModifiers};
use tokio::sync::mpsc::Sender;
use futures::{future::FutureExt, StreamExt};
use crate::ui::util::UiEvent;


/* listen_events()
 *
 * Listen for keyboard events and send the appropriate message to the UI
 * */

pub async fn listen_events(tx: Sender<UiEvent>) {
    let mut reader = EventStream::new();

    while let Some(event) = reader.next().fuse().await {

        // Keep sending until receiver is closed
        match event.unwrap() {
            // Send character keypresses to text input handler
            Key(KeyEvent{code: KeyCode::Char(ch),
                         modifiers: KeyModifiers::NONE}) => {
                if tx.send(UiEvent::Buffer(ch)).await.is_err() {
                    break;
                }
            }

            // Backspace deletes a character from the buffer.
            Key(KeyEvent{code: KeyCode::Backspace,
                         modifiers: KeyModifiers::NONE}) => {
                if tx.send(UiEvent::Del).await.is_err() {
                    break;
                }
            }

            // Enter key executes whatever is inside of the input buffer.
            Key(KeyEvent{code: KeyCode::Enter,
                         modifiers: KeyModifiers::NONE}) => {
                if tx.send(UiEvent::Execute).await.is_err() {
                    break;
                }
            }

            // Arrow keys move cursor inside of input
            Key(KeyEvent{code: KeyCode::Left,
                         modifiers: KeyModifiers::NONE}) => {
                if tx.send(UiEvent::Left).await.is_err() {
                    break;
                }
            }

            Key(KeyEvent{code: KeyCode::Right,
                         modifiers: KeyModifiers::NONE}) => {
                if tx.send(UiEvent::Right).await.is_err() {
                    break;
                }
            }

            // Keyboard Interrupt
            Key(KeyEvent{code: KeyCode::Char('c'),
                         modifiers: KeyModifiers::CONTROL}) => {
                    break;
            }

            // Some other case I'm sure I'm forgetting
            _ => {}
        }
    }
}
