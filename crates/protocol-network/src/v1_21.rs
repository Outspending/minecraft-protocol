use uuid::Uuid;

use crate::{
    buffer::{buffer::ByteBuf, varnum::VarInt},
    connection::ConnectionState,
    packet::{
        handle::Handleable,
        login::Property,
        registry::{send_registry_packets, RegistryEntry},
        result::HandledPacket,
        status::StatusResponse,
        Packet, PacketDirection, PacketSender,
    },
    register_proto,
    tcp::client::connection::MinecraftClient,
    FromNetwork, ToNetwork,
};

register_proto! {
    HandshakePacket => (0x00, Handshake, Serverbound), {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: ConnectionState
    }

    StatusRequestPacket => (0x00, Status, Serverbound),
    StatusResponsePacket => (0x00, Status, Clientbound), {
        response: StatusResponse
    }
    PingRequestPacket => (0x01, Status, Serverbound), {
        payload: i64
    }
    PingResponsePacket => (0x01, Status, Clientbound), {
        payload: i64
    }

    LoginStartPacket => (0x00, Login, Serverbound), {
        username: String,
        uuid: Uuid
    }
    LoginSuccessPacket => (0x02, Login, Clientbound), {
        uuid: Uuid,
        username: String,
        properties: Vec<Property>,
        strict_error_handling: bool
    }
    LoginAcknowledgedPacket => (0x03, Login, Serverbound),

    RegistryDataPacket => (0x07, Configuration, Clientbound), {
        registry_id: String,
        entry_count: VarInt,
        entries: Vec<RegistryEntry>
    }
}

impl Handleable for HandshakePacket {
    async fn handle(&self, client: &mut MinecraftClient) {
        let state = self.next_state;
        match state {
            ConnectionState::Status | ConnectionState::Login | ConnectionState::Transfer => {
                client.state = state;
            }
            _ => (),
        }
    }
}

impl Handleable for StatusRequestPacket {
    async fn handle(&self, client: &mut MinecraftClient) {
        client
            .send_packet(&StatusResponsePacket {
                response: StatusResponse::new(
                    "1.20.6".to_string(),
                    766,
                    20,
                    0,
                    "Wowie a Rust Status Request!".to_string(),
                ),
            })
            .await;
    }
}

impl Handleable for StatusResponsePacket {
    async fn handle(&self, _client: &mut MinecraftClient) {}
}

impl Handleable for PingRequestPacket {
    async fn handle(&self, client: &mut MinecraftClient) {
        client
            .send_packet(&PingResponsePacket {
                payload: self.payload,
            })
            .await;
    }
}

impl Handleable for PingResponsePacket {
    async fn handle(&self, _client: &mut MinecraftClient) {}
}

impl Handleable for LoginStartPacket {
    async fn handle(&self, client: &mut MinecraftClient) {
        client
            .send_packet(&LoginSuccessPacket {
                uuid: self.uuid,
                username: self.username.clone(),
                properties: vec![],
                strict_error_handling: false,
            })
            .await;
    }
}

impl Handleable for LoginSuccessPacket {
    async fn handle(&self, _client: &mut MinecraftClient) {}
}

impl Handleable for LoginAcknowledgedPacket {
    async fn handle(&self, client: &mut MinecraftClient) {
        client.state = ConnectionState::Configuration;
        send_registry_packets(client).await;
    }
}

impl Handleable for RegistryDataPacket {
    async fn handle(&self, _client: &mut MinecraftClient) {}
}
