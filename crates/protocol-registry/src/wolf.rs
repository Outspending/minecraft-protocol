use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

pub struct WolfVariant {
    pub name: String,
    pub wild_texture: String,
    pub tamed_texture: String,
    pub angry_texture: String,
    pub biomes: String,
}

impl WolfVariant {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                (
                    "wild_texture".into(),
                    NbtTag::String(self.wild_texture.clone().into()),
                ),
                (
                    "tame_texture".into(),
                    NbtTag::String(self.tamed_texture.clone().into()),
                ),
                (
                    "angry_texture".into(),
                    NbtTag::String(self.angry_texture.clone().into()),
                ),
                ("biomes".into(), NbtTag::String(self.biomes.clone().into())),
            ]),
        )
    }
}
