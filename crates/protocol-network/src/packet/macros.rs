#[macro_export]
macro_rules! register_proto {
    {
        $(
            $packet_name:ident => ($packet_id:expr, $packet_state:ident, $packet_direction:ident),
            $({
                $(
                    $field_name:ident: $field_type:ty
                ),*
            })?
        )*
    } => {
        pub async fn handle_packet(packet: &HandledPacket, client: &mut MinecraftClient) {
            let mut data = ByteBuf::new(packet.packet_data.clone());
            match (*packet.packet_id, client.state) {
                $(
                    ($packet_id, ConnectionState::$packet_state) if PacketDirection::$packet_direction == PacketDirection::Serverbound => {
                        let packet = $packet_name::from_network(&mut data);
                        println!("[{:?}] Handling packet: {:?}", client.state, packet);
                        packet.handle(client).await;
                        return;
                    }
                ),*
                _ => ()
            };

            println!("[{:?}] Unknown packet: {:?}", client.state, packet.packet_id);
        }

        $(
            #[derive(Debug)]
            pub struct $packet_name {
                $($(
                    pub $field_name: $field_type
                ),*)?
            }

            impl ToNetwork for $packet_name {
                fn to_network(&self, buf: &mut ByteBuf) {
                    $($(
                        self.$field_name.to_network(buf);
                    )?)*
                }
            }

            impl FromNetwork for $packet_name {
                fn from_network(buf: &mut ByteBuf) -> Self {
                    let packet = Self {
                        $($(
                            $field_name: <$field_type>::from_network(buf)
                        ),*)?
                    };

                    packet
                }
            }

            impl Packet for $packet_name {
                fn id(&self) -> i16 {
                    $packet_id
                }
            }
        )*
    };
}
