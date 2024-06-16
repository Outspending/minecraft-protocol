use tokio::net::{TcpListener, TcpStream};

use crate::{connection::Connection, tcp::client::connection::MinecraftClient};

pub trait ServerConnection: Connection {
    async fn new_client(&self, socket: TcpStream);
}

pub struct MinecraftServerConnection<'a> {
    listener: TcpListener,
    pub host: &'a str,
    pub port: u16,
    pub connected: bool,
}

impl<'a> Connection for MinecraftServerConnection<'a> {
    async fn connect(&mut self) {
        self.connected = true;
        loop {
            if !self.connected {
                break;
            } else {
                let (socket, _) = self.listener.accept().await.unwrap();
                self.new_client(socket).await; // TODO: Make this a separate task
            }
        }
    }

    fn disconnect(&mut self) {
        self.connected = false;
    }
}

impl<'a> ServerConnection for MinecraftServerConnection<'a> {
    async fn new_client(&self, socket: TcpStream) {
        let mut client_connection = MinecraftClient::new(socket);
        client_connection.connect().await;
    }
}

pub struct MinecraftServer<'a> {
    pub connection: MinecraftServerConnection<'a>,
}

impl<'a> MinecraftServer<'a> {
    pub async fn new(host: &'a str, port: u16) -> Self {
        let listener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();
        let connection = MinecraftServerConnection {
            listener,
            host,
            port,
            connected: false,
        };

        Self { connection }
    }

    pub async fn start(&mut self) {
        self.connection.connect().await;
    }
}
