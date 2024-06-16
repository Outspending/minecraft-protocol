use crate::tcp::client::connection::MinecraftClient;

pub trait Handleable {
    async fn handle(&self, client: &mut MinecraftClient);
}
