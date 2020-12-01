use crate::Buffer;

pub trait Edit {
    fn edit(&self, buffer: &mut Buffer) -> Result<(), EditError>;
}

pub enum EditError {}
