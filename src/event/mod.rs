use anyhow::Result;

pub struct InputBuffer {
    pub message_buffer: String,
}

impl InputBuffer {
    pub fn new() -> InputBuffer {
        InputBuffer{ message_buffer: String::new() }
    }

    pub fn execute(&self) -> Result<()> {
        if self.message_buffer.starts_with('/') {
            // TODO Execute control commands
        } else {
            // TODO Send irc message
        }
        Ok(())
    }
}
