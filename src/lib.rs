#[cfg(test)]
macro_rules! re {
    ($regex: expr) => {
        crate::re::Re::from_str($regex).unwrap()
    };
}

mod buffer;
pub mod ed;
mod edit;
mod re;

pub use buffer::Buffer;
pub use edit::{Edit, EditError};

pub(crate) trait Parsable: Sized {
    fn parse(input: &str) -> nom::IResult<&str, Self>;
}
