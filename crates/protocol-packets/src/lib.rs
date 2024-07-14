use protocol_buf::buffer::{NormalBuffer, PacketBuffer};

pub mod macros;

/// This trait defines all packets that can be send between the client or the server.
///
/// The `[ClientboundPacket]` and the `[ServerboundPacket]` traits are used to define the packets that can be send between the client and the server.
///
/// # Examples
/// ```rust
/// use protocol::Packet;
///
/// struct HandshakePacket {
///    pub protocol_version: i32,
///    pub server_address: String,
///    pub server_port: u16,
///   pub next_state: i32,
/// }
///
/// impl Packet for HandshakePacket {
///    fn id(&self) -> i32 {
///       0x00
///   }
/// }
pub trait Packet {
    fn id(&self) -> i32;
}

/// Defines a packet that can be sent from the server to the client.
///
/// This trait implements the `[Packet]` and the `[ToNetwork]` trait.
///
/// # Examples
/// ```rust
/// use protocol::ClientboundPacket;
///
/// struct HandshakePacket {
///   pub protocol_version: i32,
///   pub server_address: String,
///   pub server_port: u16,
///   pub next_state: i32,
/// }
///
/// impl ClientboundPacket for HandshakePacket {
///   fn id(&self) -> i32 {
///     0x00
///   }
///
///   fn write_packet(&self, buffer: NormalBuffer) -> PacketBuffer {
///     buffer.write(self.protocol_version);
///     buffer.write(self.server_address.clone());
///     buffer.write(self.server_port);
///     buffer.write(self.next_state);
///     buffer
///   }
/// }
/// ```
pub trait ClientboundPacket: Packet {
    fn write_packet(&self, buffer: NormalBuffer) -> PacketBuffer;
}

/// Defines a packet that can be sent from the client to the server.
///
/// This trait implements the `[Packet]` and the `[FromNetwork]` trait.
///
/// # Examples
/// ```rust
/// use protocol::ServerboundPacket;
///
/// struct HandshakePacket {
///   pub protocol_version: i32,
///   pub server_address: String,
///   pub server_port: u16,
///   pub next_state: i32,
/// }
///
/// impl ServerboundPacket for HandshakePacket {
///   fn id(&self) -> i32 {
///     0x00
///   }
///
///   fn read_packet(buffer: PacketBuffer) -> Self {
///     let protocol_version: i32 = buffer.read();
///     let server_address: String = buffer.read();
///     let server_port: u16 = buffer.read();
///     let next_state: i32 = buffer.read();
///     HandshakePacket {
///       protocol_version,
///       server_address,
///       server_port,
///       next_state,
///     }
///   }
/// }
pub trait ServerboundPacket: Packet {
    fn read_packet(buffer: NormalBuffer) -> Self;
}
