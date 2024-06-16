use buffer::buffer::ByteBuf;

pub mod buffer;
pub mod connection;
pub mod packet;
pub mod tcp;
pub mod v1_21;
pub mod identifier;
pub mod position;

pub trait ToNetwork {
    fn to_network(&self, buf: &mut ByteBuf);
}

pub trait FromNetwork {
    fn from_network(buf: &mut ByteBuf) -> Self;
}
