use super::{Action, Tui};

pub enum KeyBuffer {
    Push(char),
    Clear,
}

impl Action for KeyBuffer {
    fn invoke(&self, tui: &mut Tui) -> eyre::Result<()> {
        match self {
            KeyBuffer::Push(ch) => tui.key_buffer.push(*ch),
            KeyBuffer::Clear => tui.key_buffer.clear(),
        }

        tui.draw_cmd()?;
        Ok(())
    }
}
