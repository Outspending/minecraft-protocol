use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

#[derive(Debug, Clone)]
pub struct Banner {
    pub name: String,
    pub asset_id: String,
    pub translation_key: String,
}

impl Banner {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                (
                    "asset_id".into(),
                    NbtTag::String(self.asset_id.clone().into()),
                ),
                (
                    "translation_key".into(),
                    NbtTag::String(self.translation_key.clone().into()),
                ),
            ]),
        )
    }
}
