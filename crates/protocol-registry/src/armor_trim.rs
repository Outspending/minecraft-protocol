use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

#[derive(Debug, Clone)]
pub struct ArmorTrimMaterial<'a> {
    pub name: &'a str,
    pub asset_name: &'a str,
    pub ingredient: &'a str,
    pub item_model_index: f32,
    pub override_armor_materials: Option<Vec<&'a str>>, // Isn't implemented in the NBT
    pub description: &'a str,
}

impl<'a> ArmorTrimMaterial<'a> {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                (
                    "asset_name".into(),
                    NbtTag::String(self.asset_name.into()),
                ),
                (
                    "ingredient".into(),
                    NbtTag::String(self.ingredient.into()),
                ),
                (
                    "item_model_index".into(),
                    NbtTag::Float(self.item_model_index),
                ),
                (
                    "description".into(),
                    NbtTag::String(self.description.into()),
                ),
            ]),
        )
    }
}

#[derive(Debug, Clone)]
pub struct ArmorTrimPattern<'a> {
    pub name: &'a str,
    pub asset_id: &'a str,
    pub template_item: &'a str,
    pub description: &'a str,
    pub decal: u8,
}

impl<'a> ArmorTrimPattern<'a> {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            self.name.clone().into(),
            NbtCompound::from_values(vec![
                (
                    "asset_id".into(),
                    NbtTag::String(self.asset_id.into()),
                ),
                (
                    "template_item".into(),
                    NbtTag::String(self.template_item.into()),
                ),
                (
                    "description".into(),
                    NbtTag::String(self.description.into()),
                ),
                ("decal".into(), NbtTag::Byte(self.decal as i8)),
            ]),
        )
    }
}
