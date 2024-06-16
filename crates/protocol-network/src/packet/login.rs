use crate::{buffer::buffer::ByteBuf, FromNetwork, ToNetwork};

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signed: bool,
    pub signature: Option<String>,
}

impl ToNetwork for Property {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write_string(self.name.clone());
        buf.write_string(self.value.clone());
        buf.write_bool(self.signed);
        if let Some(signature) = &self.signature {
            buf.write_string(signature.clone());
        }
    }
}

impl FromNetwork for Property {
    fn from_network(buf: &mut ByteBuf) -> Self {
        let name = buf.read_string();
        let value = buf.read_string();
        let signed = buf.read_bool();
        let signature = if signed {
            Some(buf.read_string())
        } else {
            None
        };
        Self {
            name,
            value,
            signed,
            signature,
        }
    }
}
