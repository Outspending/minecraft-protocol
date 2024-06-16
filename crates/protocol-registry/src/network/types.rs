use simdnbt::owned::NbtTag;

pub enum TemperatureModifier {
    None,
    Frozen,
}

impl TemperatureModifier {
    pub fn to_nbt(&self) -> NbtTag {
        match self {
            TemperatureModifier::None => NbtTag::String("none".into()),
            TemperatureModifier::Frozen => NbtTag::String("frozen".into()),
        }
    }
}

pub enum DimensionEffects {
    Overworld,
    Nether,
    End,
}

impl DimensionEffects {
    pub fn to_nbt(&self) -> NbtTag {
        match self {
            DimensionEffects::Overworld => NbtTag::String("minecraft:overworld".into()),
            DimensionEffects::Nether => NbtTag::String("minecraft:the_nether".into()),
            DimensionEffects::End => NbtTag::String("minecraft:the_end".into()),
        }
    }
}
