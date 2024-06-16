use crate::{
    buffer::{buffer::ByteBuf, varnum::VarInt},
    FromNetwork, ToNetwork,
};

pub trait Connection {
    async fn connect(&mut self);

    fn disconnect(&mut self);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ConnectionState {
    #[default]
    Handshake,
    Status,
    Login,
    Transfer,
    Configuration,
    Play,
}

impl ConnectionState {
    pub fn get_id(&self) -> i32 {
        match self {
            ConnectionState::Handshake => 0,
            ConnectionState::Status => 1,
            ConnectionState::Login => 2,
            ConnectionState::Transfer => 3,
            ConnectionState::Configuration => 4,
            ConnectionState::Play => 5,
        }
    }
}

impl FromNetwork for ConnectionState {
    fn from_network(buf: &mut ByteBuf) -> Self {
        match *buf.read_varint() {
            0 => ConnectionState::Handshake,
            1 => ConnectionState::Status,
            2 => ConnectionState::Login,
            3 => ConnectionState::Transfer,
            4 => ConnectionState::Configuration,
            5 => ConnectionState::Play,
            _ => ConnectionState::Handshake,
        }
    }
}

impl ToNetwork for ConnectionState {
    fn to_network(&self, buf: &mut ByteBuf) {
        let varint = VarInt::from(self.get_id());
        varint.to_network(buf);
    }
}
