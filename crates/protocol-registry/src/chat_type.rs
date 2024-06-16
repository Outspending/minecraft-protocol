use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

pub struct ChatType {
    pub name: String,
    pub translation_key: String,
    pub style: Option<NbtCompound>,
    pub parameters: String,
}

impl ChatType {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                (
                    "translation_key".into(),
                    NbtTag::String(self.translation_key.clone().into()),
                ),
                (
                    "parameters".into(),
                    NbtTag::String(self.parameters.clone().into()),
                ),
            ]),
        )
    }
}
