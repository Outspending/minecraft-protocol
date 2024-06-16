use uuid::Uuid;

use crate::{
    buffer::{buffer::ByteBuf, varnum::VarInt}, connection::ConnectionState, packet::{
        handle::Handleable,
        login::Property,
        registry::{send_registry_packets, RegistryEntry},
        result::HandledPacket,
        status::StatusResponse,
        Packet, PacketDirection, PacketSender,
    }, position::Position, register_proto, tcp::client::connection::MinecraftClient, FromNetwork, ToNetwork
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
        entries: Vec<RegistryEntry>
    }
    ConfigurationDisconnectPacket => (0x02, Configuration, Serverbound), {
        reason: String
    }
    FinishConfigurationPacket => (0x03, Configuration, Clientbound),
    AcknowledgeFinishConfigurationPacket => (0x03, Configuration, Serverbound),

    LoginPlayPacket => (0x2B, Play, Clientbound), {
        entity_id: i32,
        is_hardcore: bool,
        dimension_names: Vec<String>,
        max_players: VarInt,
        view_distance: VarInt,
        simulation_distance: VarInt,
        reduced_debug_info: bool,
        enable_respawn_screen: bool,
        do_limited_crafting: bool,
        dimension_type: VarInt,
        dimension_name: String,
        hashed_seed: i64,
        game_mode: u8,
        previous_game_mode: i8,
        is_debug: bool,
        is_flat: bool,
        has_death_location: bool,
        death_dimension_name: Option<String>,
        death_location: Option<Position>,
        portal_cooldown: VarInt,
        enforces_secure_chat: bool
    }
    GameEventPacket => (0x22, Play, Clientbound), {
        event: u8,
        value: f32
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

impl Handleable for ConfigurationDisconnectPacket {
    async fn handle(&self, _client: &mut MinecraftClient) {}
}

impl Handleable for FinishConfigurationPacket {
    async fn handle(&self, _client: &mut MinecraftClient) {}
}

impl Handleable for AcknowledgeFinishConfigurationPacket {
    async fn handle(&self, client: &mut MinecraftClient) {
        client.state = ConnectionState::Play;
        client.send_packet(&LoginPlayPacket {
            entity_id: 1,
            is_hardcore: false,
            dimension_names: vec!["minecraft:overworld".to_string()],
            max_players: VarInt::from(20),
            view_distance: VarInt::from(12),
            simulation_distance: VarInt::from(12),
            reduced_debug_info: false,
            enable_respawn_screen: false,
            do_limited_crafting: false,
            dimension_type: VarInt::from(0),
            dimension_name: "minecraft:overworld".to_string(),
            hashed_seed: 0,
            game_mode: 0,
            previous_game_mode: -1,
            is_debug: false,
            is_flat: false,
            has_death_location: false,
            death_dimension_name: None,
            death_location: None,
            portal_cooldown: VarInt::from(0),
            enforces_secure_chat: false,
        }).await;   
        client.send_packet(&GameEventPacket {
            event: 13,
            value: 0.0,
        }).await;
    }
}

impl Handleable for LoginPlayPacket {
    async fn handle(&self, _client: &mut MinecraftClient) {}
}

impl Handleable for GameEventPacket {
    async fn handle(&self, _client: &mut MinecraftClient) {}
}