use std::fmt::Display;

use crate::{
    buffer::{buffer::ByteBuf, varnum::VarInt},
    tcp::client::connection::MinecraftClient,
};

pub struct ConnectionResult {
    pub buf: ByteBuf,
}

impl ConnectionResult {
    pub fn new(buf: &[u8]) -> Self {
        Self {
            buf: ByteBuf::new(buf.to_vec()),
        }
    }

    pub fn handle_packet(&mut self) -> HandledPacket {
        HandledPacket {
            packet_length: self.buf.read_varint(),
            packet_id: self.buf.read_varint(),
            packet_data: self.buf.get_rest(),
        }
    }
}

pub struct HandledPacket {
    pub packet_length: VarInt,
    pub packet_id: VarInt,
    pub packet_data: Vec<u8>,
}

impl Display for HandledPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HandledPacket: {{ length: {}, id: {}, data: {:?} }}",
            self.packet_length, self.packet_id, self.packet_data
        )
    }
}

impl HandledPacket {
    pub async fn handle_packet(&self, client: &mut MinecraftClient) {
        crate::v1_21::handle_packet(&self, client).await;
    }
}
