/* Read and handle keypresses received from crossterm's EventStream */
use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers},
};
use futures::{future::FutureExt, StreamExt};

/* First step in input pipeline -- receives terminal events and
 * sends them to the appropriate handler.
 *
 * Panics --
 * On unknown keyboard input, panics.
 * */

pub async fn receive_terminal_events() {
    let mut reader = EventStream::new();

    loop {
        let event = reader.next().fuse();
        let maybe_event = event.await.unwrap().unwrap();
        match maybe_event {
            // Any character keypress, unmodified.
            Event::Key(KeyEvent{code: KeyCode::Char(ch),
                                modifiers: KeyModifiers::NONE}) => {
                println!("{} was pressed.", ch);
            }
            // Keyboard Interrupt
            Event::Key(KeyEvent{code: KeyCode::Char('c'),
                                modifiers: KeyModifiers::CONTROL}) => {
                println!("Caught keyboard interrupt.");
                break;
            }
            _ => {
                panic!("Unhandled terminal event");
            }
        }
    }
}
