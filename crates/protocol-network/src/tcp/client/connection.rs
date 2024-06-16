use std::ops::Deref;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    buffer::{buffer::ByteBuf, varnum::VarInt},
    connection::{Connection, ConnectionState},
    packet::{result::ConnectionResult, Packet, PacketSender},
};

pub struct MinecraftClient {
    listener: TcpStream,
    pub connected: bool,
    pub state: ConnectionState,
}

impl Deref for MinecraftClient {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.listener
    }
}

impl Connection for MinecraftClient {
    async fn connect(&mut self) {
        loop {
            let mut buf = [0u8; 1024];
            let read = self.listener.read(&mut buf).await.unwrap();
            if !self.connected || read == 0 {
                break;
            }

            let mut result = ConnectionResult::new(&buf[..read]);
            let packet_result = result.handle_packet();
            packet_result.handle_packet(self).await;
        }
    }

    fn disconnect(&mut self) {}
}

impl PacketSender for MinecraftClient {
    async fn send_packet<T: Packet>(&mut self, packet: &T) {
        let varint_id = VarInt::from(packet.id() as i32);
        let mut buf = ByteBuf::new_empty();
        buf.write_varint(varint_id);
        packet.to_network(&mut buf);

        let packet_length = VarInt::from(buf.len() as i32);
        buf.write_varint(packet_length);
        buf.get_mut()
            .rotate_right(packet_length.get_size_in_bytes());

        println!(
            "[{:?}] Sending packet: {:?}",
            self.listener.peer_addr().unwrap(),
            buf.get_ref()
        );

        self.listener.write(buf.get_ref()).await.unwrap();
    }
}

impl MinecraftClient {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            listener: socket,
            connected: true,
            state: ConnectionState::default(),
        }
    }
}
