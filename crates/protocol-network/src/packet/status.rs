use serde::{Deserialize, Serialize};

use crate::{buffer::buffer::ByteBuf, FromNetwork, ToNetwork};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    players: Players,
    version: Version,
    description: String,
}

impl FromNetwork for StatusResponse {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let string = buf.read_string();
        serde_json::from_str(&string).unwrap()
    }
}

impl ToNetwork for StatusResponse {
    fn to_network(&self, buf: &mut ByteBuf) {
        let string = serde_json::to_string(self).unwrap();
        buf.write_string(string);
    }
}

impl StatusResponse {
    pub fn new(name: String, protocol: i32, max: i32, online: i32, description: String) -> Self {
        Self {
            players: Players { max, online },
            version: Version { name, protocol },
            description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    name: String,
    protocol: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Players {
    max: i32,
    online: i32,
}
