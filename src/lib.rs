#[macro_use] extern crate log;
extern crate byteorder;
extern crate lycan_serialize;

pub mod messages {
    pub use lycan_serialize::*;
}
pub mod net;
