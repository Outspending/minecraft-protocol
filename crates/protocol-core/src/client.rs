use protocol_buf::{
    buffer::{Buffer, PacketBuffer},
    compression::CompressionData,
};
use tokio::{io::AsyncReadExt, net::TcpStream};

/// Represents a client connection.
///
/// The TCP stream usually is grabbed from the server connection. This is rarely created manually. If so, it is usually for testing purposes.
/// Its not recommended to create this struct manually yourself.
///
/// # Fields
/// - `listener` - The TCP stream that listens for incoming data.
pub struct ClientConnection {
    listener: TcpStream,
}

/// Represents a client connection.
///
/// This struct is handling the whole client connection. If you are looking for its connection, check `[ClientConnection]`.
///
/// Again, same as `[ClientConnection]`, this is usally created by the server connection. This is rarely created manually.
/// If you are creating this manually there might be something wrong.
///
/// # Fields
/// - `connection` - The client connection.
/// - `compression` - The compression data, which includes threshold and compression type.
pub struct Client {
    pub connection: ClientConnection,
    pub compression: CompressionData,
}

impl Client {
    /// Creates a new `[Client]` instance with the given TCP stream and compression data.
    ///
    /// The TCP stream is usually created by the server connection. This is rarely created manually.
    /// The compression data is usually created by the server connection. This is rarely created manually.
    pub const fn new(listener: TcpStream, compression: CompressionData) -> Self {
        Self {
            connection: ClientConnection { listener },
            compression,
        }
    }

    /// This method is used to "start" the client connection. This is where the client connection will start listening for incoming data aka packets.
    ///
    /// Here the bytes are being converted into a `[PacketBuffer]`, which is a custom `[Buffer]` inside `protocol_buf`.
    /// This makes it easier to read and write packets.
    ///
    /// # Note
    /// If you are using `[ServerConnection]` to accept connections, if you aren't defining the callback parameter yourself, this is automatically called within the API.
    pub async fn start(&mut self) {
        loop {
            let mut buffer = [0_u8; 1024];
            match self.connection.listener.read(&mut buffer).await {
                Ok(0) => {
                    println!("Client Disconnected...");
                    break;
                }
                Ok(n) => {
                    let buffer = buffer[..n].to_vec();
                    if let Some(packet_data) = PacketBuffer::new(buffer, &self.compression) {
                        println!(
                            "Packet Length: {} // Packet ID: {}",
                            *packet_data.packet_length, *packet_data.packet_id
                        );
                        println!("Received: {:?}", packet_data.get_ref());
                    }
                }
                Err(e) => {
                    println!("Failed to read from socket; err = {:?}", e);
                    break;
                }
            }
        }
    }
}
