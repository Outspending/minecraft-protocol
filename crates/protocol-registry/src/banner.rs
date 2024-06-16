use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

#[derive(Debug, Clone)]
pub struct BannerPattern<'a> {
    pub name: &'a str,
    pub asset_id: &'a str,
    pub translation_key: &'a str,
}

impl<'a> BannerPattern<'a> {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                (
                    "asset_id".into(),
                    NbtTag::String(self.asset_id.into()),
                ),
                (
                    "translation_key".into(),
                    NbtTag::String(self.translation_key.into()),
                ),
            ]),
        )
    }
}
