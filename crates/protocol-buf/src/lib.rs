use std::io::Cursor;

pub mod buffer;
pub mod compression;
pub(crate) mod macros;
pub mod types;

/// Defines a trait for an object that can be written to a `[Buffer]`
pub trait ToNetwork {
    fn to_network(&self) -> Vec<u8>;
}

/// Defines a trait for an object that can be read from a `[Buffer]`
pub trait FromNetwork: Sized {
    fn from_network(buffer: &mut Cursor<Vec<u8>>) -> Self;
}
