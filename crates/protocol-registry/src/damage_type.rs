use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

pub struct DamageType {
    pub name: String,
    pub message_id: String,
    pub scaling: String,
    pub exhaustion: f32,
    pub effects: Option<String>,
    pub death_message_type: Option<String>,
}

impl DamageType {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                (
                    "message_id".into(),
                    NbtTag::String(self.message_id.clone().into()),
                ),
                (
                    "scaling".into(),
                    NbtTag::String(self.scaling.clone().into()),
                ),
                ("exhausion".into(), NbtTag::Float(self.exhaustion)),
            ]),
        )
    }
}
