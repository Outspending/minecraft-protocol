use protocol_core::server::MinecraftServer;

#[tokio::main]
async fn main() {
    let mut server = MinecraftServer::new("127.0.0.1", 25565).await;
    tokio::spawn(async move {
        server.accept_connections().await;
    });

    println!("Server started! Press Ctrl-C to stop.");
    tokio::signal::ctrl_c().await.unwrap();
}
