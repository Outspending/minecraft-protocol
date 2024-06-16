use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

pub struct DamageType<'a> {
    pub name: &'a str,
    pub message_id: &'a str,
    pub scaling: &'a str,
    pub exhaustion: f32,
    pub effects: Option<&'a str>,
    pub death_message_type: Option<&'a str>,
}

impl<'a> DamageType<'a> {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                (
                    "scaling".into(),
                    NbtTag::String(self.scaling.into()),
                ),
                ("exhaustion".into(), NbtTag::Float(self.exhaustion)),
                (
                    "message_id".into(),
                    NbtTag::String(self.message_id.into()),
                ),
            ]),
        )
    }
}
