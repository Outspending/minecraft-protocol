use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

pub struct ChatType {
    pub name: String,
    pub chat: ChatDecoration,
    pub narrator: ChatDecoration
}

impl ChatType {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                ("chat".into(), self.chat.to_nbt()),
                ("narration".into(), self.narrator.to_nbt())
            ]),
        )
    }
}

pub struct ChatDecoration {
    pub name: String,
    pub translation_key: String,
    pub parameters: Vec<String>,
}

impl ChatDecoration {
    pub fn to_nbt(&self) -> NbtTag {
        NbtTag::Compound(
            NbtCompound::from_values(vec![
                (
                    "name".into(),
                    NbtTag::String(self.name.clone().into()),
                ),
                (
                    "translation_key".into(),
                    NbtTag::String(self.translation_key.clone().into()),
                ),
                (
                    "parameters".into(),
                    NbtTag::List(self.parameters.clone().into()),
                ),
            ]),
        )
    }
}