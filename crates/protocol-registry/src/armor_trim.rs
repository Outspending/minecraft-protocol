use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

#[derive(Debug, Clone)]
pub struct ArmorTrimMaterial {
    pub name: String,
    pub asset_name: String,
    pub ingredient: String,
    pub item_model_index: f32,
    pub override_armor_materials: Option<Vec<String>>, // Isn't implemented in the NBT
    pub description: String,
}

impl ArmorTrimMaterial {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                (
                    "asset_name".into(),
                    NbtTag::String(self.asset_name.clone().into()),
                ),
                (
                    "ingredient".into(),
                    NbtTag::String(self.ingredient.clone().into()),
                ),
                (
                    "item_model_index".into(),
                    NbtTag::Float(self.item_model_index),
                ),
                (
                    "description".into(),
                    NbtTag::String(self.description.clone().into()),
                ),
            ]),
        )
    }
}

#[derive(Debug, Clone)]
pub struct ArmorTrimPattern {
    pub name: String,
    pub asset_id: String,
    pub template_item: String,
    pub description: String,
    pub decal: u8,
}

impl ArmorTrimPattern {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            self.name.clone().into(),
            NbtCompound::from_values(vec![
                (
                    "asset_id".into(),
                    NbtTag::String(self.asset_id.clone().into()),
                ),
                (
                    "template_item".into(),
                    NbtTag::String(self.template_item.clone().into()),
                ),
                (
                    "description".into(),
                    NbtTag::String(self.description.clone().into()),
                ),
                ("decal".into(), NbtTag::Byte(self.decal as i8)),
            ]),
        )
    }
}
