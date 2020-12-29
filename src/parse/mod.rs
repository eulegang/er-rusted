mod cmd;
mod command;
mod syspoint;

mod address;
mod offset;
mod point;

pub(crate) trait Parsable: Sized {
    fn parse(input: &str) -> nom::IResult<&str, Self>;
}
