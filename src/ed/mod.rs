//!
//! How to parse ed commands and interact with buffers
//!

pub(crate) mod addr;
pub(crate) mod cmd;
pub(crate) mod parse;
pub(crate) mod re;
pub(crate) mod resolve;
pub(crate) mod syspoint;

pub(crate) mod prelude {
    use super::*;

    pub use addr::{Address, Offset, Point};
    pub use cmd::Command;
    pub use re::{Pat, Re};
    pub use resolve::{LineResolver, RangeResolver};
    pub use syspoint::{Cmd, Sourcer, Syncer, SysPoint};
}
