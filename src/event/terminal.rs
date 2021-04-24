/* Read and handle keypresses received from crossterm's EventStream */
use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers},
};
use futures::{future::FutureExt, StreamExt};

pub async fn receive_terminal_events() {
    let mut reader = EventStream::new();

    loop {
        let event = reader.next().fuse();
        match event.await {
            Some(Ok(event)) => {
                if event == Event::Key(
                    KeyEvent{code: KeyCode::Char('c'),
                             modifiers: KeyModifiers::CONTROL}) {
                    println!("Caught interrupt");
                    break;
                } else {
                    println!("Character {:?} typed.", event)
                }
            }
            _ => panic!("idk what I'm doing")
        }
    }
}
