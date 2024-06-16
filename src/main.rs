use protocol_network::tcp::server::connection::MinecraftServer;

#[tokio::main]
async fn main() {
    let mut minecraft_server = MinecraftServer::new("127.0.0.1", 25565).await;
    minecraft_server.start().await;
}
