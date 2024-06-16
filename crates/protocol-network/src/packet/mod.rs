use handle::Handleable;

use crate::{FromNetwork, ToNetwork};

pub mod handle;
pub mod login;
pub mod macros;
pub mod registry;
pub mod result;
pub mod status;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketDirection {
    Serverbound,
    Clientbound,
}

pub trait Packet: Handleable + FromNetwork + ToNetwork + Sized {
    fn id(&self) -> i16;
}

pub trait PacketSender {
    async fn send_packet<T: Packet>(&mut self, packet: &T);
}
