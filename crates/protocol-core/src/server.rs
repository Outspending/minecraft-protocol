use std::{
    future::Future,
    sync::atomic::{AtomicBool, Ordering},
};

use protocol_buf::compression::{CompressionData, CompressionType};
use tokio::net::TcpListener;

use crate::client::Client;

/// Represents the `[MinecraftServer]` Connection.
///
/// This struct is responsible for accepting incoming connections from clients.
///
/// This struct also contains the compression threshold for the server.
/// This is the size at which all packets should be compressed. Any packets smaller than this size will NOT be compressed.
///
/// # Fields
/// - `stream` - The TCP listener that listens for incoming connections.
/// - `compression_threshold` - The threshold at which packets should be compressed.
/// - `is_running` - A flag that indicates if the server is running.
///
/// # Examples
/// ```rust
/// use tokio::net::TcpListener;
/// use protocol_core::server::ServerConnection;
///
/// #[tokio::main]
/// async fn main() {
///    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();
///    let mut server = ServerConnection::new(listener);
///    server.accept_connections(|client| async move {
///     client.start().await;
///    });
/// }
/// ```
pub struct ServerConnection {
    stream: TcpListener,
    pub compression_threshold: i32,
    pub is_running: AtomicBool,
}

impl ServerConnection {
    /// Creates a new `[ServerConnection]` instance with the given TCP listener.
    ///
    /// The TCP listener is usally created by binding to an address and port inside `[MinecraftServer]` instead of creating this object yourself.
    ///
    /// # Examples
    /// ```rust
    /// use tokio::net::TcpListener;
    /// use protocol_core::server::ServerConnection;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();
    ///     let server = ServerConnection::new(listener);
    /// }
    /// ```
    pub const fn new(stream: TcpListener) -> Self {
        Self {
            stream,
            compression_threshold: 256,
            is_running: AtomicBool::new(true),
        }
    }

    /// This method accepts incoming connections from clients.
    ///
    /// This method will call whenever a client tries to connect with the server. This is usually started with the Handshake Packet.
    ///
    /// # Parameters
    /// - `callback` - The callback to call when a client connects.
    pub async fn accept_connections<T, F>(&mut self, mut callback: T)
    where
        T: FnMut(Client) -> F + Send + Clone + Copy + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        while self.is_running.load(Ordering::SeqCst) {
            if let Ok((socket, _)) = self.stream.accept().await {
                let client = Client::new(
                    socket,
                    CompressionData::new(self.compression_threshold, CompressionType::None),
                );

                tokio::spawn(async move {
                    callback(client).await;
                });
            }
        }
    }

    /// Stops the server from accepting new connections.
    ///
    /// This method also will not stop all the existing connections.
    /// Therefore, you'll have to manually kick all existing connections or they will be timed out after 15 seconds.
    ///
    /// # Examples
    /// ```rust
    /// use tokio::net::TcpListener;
    /// use protocol_core::server::ServerConnection;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();
    ///    let mut server = ServerConnection::new(listener);
    ///    server.stop();
    /// }
    /// ```
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    /// This method sets the compression threshold for all new connections.
    ///
    /// This WILL not affect existing connections. If you are looking to change the compression threshold for all existing connections. You'll have to manually change it yourself.
    ///
    /// # Parameters
    /// - `threshold` - The threshold at which packets should be compressed.
    ///
    /// # Examples
    /// ```rust
    /// use tokio::net::TcpListener;
    /// use protocol_core::server::ServerConnection;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();
    ///    let mut server = ServerConnection::new(listener);
    ///    server.set_compression_threshold(25);
    /// }
    /// ```
    pub fn set_compression_threshold(&mut self, threshold: i32) {
        self.compression_threshold = threshold;
    }
}

/// Represents the main Minecraft Server object.
///
/// This struct holds everything needed to run the Minecraft Server. This includes the server connection and the server itself.
///
/// # Fields
/// - `connection` - The server connection that listens for incoming connections.
///
/// # Examples
/// ```rust
/// use protocol_core::server::MinecraftServer;
///
/// #[tokio::main]
/// async fn main() {
///    let mut server = MinecraftServer::new("127.0.0.1", 25565).await;
/// }
/// ```
pub struct MinecraftServer {
    pub connection: ServerConnection,
}

impl MinecraftServer {
    /// Creates a new `[MinecraftServer]` instance with the given address and port.
    ///
    /// This method will bind to the given address and port and start listening for incoming connections.
    ///
    /// # Parameters
    /// - `addr` - The address to bind to.
    /// - `port` - The port to bind to.
    ///
    /// # Examples
    /// ```rust
    /// use protocol_core::server::MinecraftServer;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let mut server = MinecraftServer::new("127.0.0.1", 25565).await;
    /// }
    /// ```
    pub async fn new(addr: &str, port: u16) -> Self {
        Self {
            connection: ServerConnection::new(
                TcpListener::bind(format!("{}:{}", addr, port))
                    .await
                    .unwrap(),
            ),
        }
    }

    /// This method accepts incoming connections from clients.
    ///
    /// This method will call whenever a client tries to connect with the server. This is usually started with the Handshake Packet.
    ///
    /// # Examples
    /// ```rust
    /// use protocol_core::server::MinecraftServer;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut server = MinecraftServer::new("127.0.0.1", 25565).await;
    ///     server.accept_connections().await;
    /// }
    /// ```
    pub async fn accept_connections(&mut self) {
        self.connection
            .accept_connections(|mut connection| async move { connection.start().await })
            .await;
    }

    /// Stops the server from accepting new connections.
    ///
    /// This method also will not stop all the existing connections.
    /// Therefore, you'll have to manually kick all existing connections or they will be timed out after 15 seconds.
    ///
    /// # Examples
    /// ```rust
    /// use protocol_core::server::MinecraftServer;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut server = MinecraftServer::new("127.0.0.1", 25565).await;
    ///     server.stop();
    /// }
    /// ```
    pub fn stop(&mut self) {
        self.connection.stop();
    }

    /// This method sets the compression threshold for all new connections.
    ///
    /// This WILL not affect existing connections. If you are looking to change the compression threshold for all existing connections. You'll have to manually change it yourself.
    ///
    /// # Parameters
    /// - `threshold` - The threshold at which packets should be compressed.
    ///
    /// # Examples
    /// ```rust
    /// use protocol_core::server::MinecraftServer;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut server = MinecraftServer::new("127.0.0.1", 25565).await;
    ///     server.set_compression_threshold(25);
    /// }
    /// ```
    pub fn set_compression_threshold(&mut self, threshold: i32) {
        self.connection.set_compression_threshold(threshold);
    }
}
