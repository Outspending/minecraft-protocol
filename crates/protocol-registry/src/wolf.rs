use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

pub struct WolfVariant<'a> {
    pub name: &'a str,
    pub wild_texture: &'a str,
    pub tamed_texture: &'a str,
    pub angry_texture: &'a str,
    pub biomes: &'a str,
}

impl<'a> WolfVariant<'a> {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                (
                    "wild_texture".into(),
                    NbtTag::String(self.wild_texture.into()),
                ),
                (
                    "tame_texture".into(),
                    NbtTag::String(self.tamed_texture.into()),
                ),
                (
                    "angry_texture".into(),
                    NbtTag::String(self.angry_texture.into()),
                ),
                ("biomes".into(), NbtTag::String(self.biomes.into())),
            ]),
        )
    }
}
