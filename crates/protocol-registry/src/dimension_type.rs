use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

use crate::network::types::DimensionEffects;

pub struct DimensionType<'a> {
    pub name: &'a str,
    pub fixed_time: Option<i64>,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub ultrawarm: bool,
    pub natural: bool,
    pub coordinate_scale: f32,
    pub bed_works: bool,
    pub respawn_anchor_works: bool,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub infiniburn: &'a str,
    pub effects: DimensionEffects,
    pub ambient_light: f32,
    pub piglin_safe: bool,
    pub has_raids: bool,
    pub monster_spawn_light_level: i32,
    pub monster_spawn_block_light_limit: i32,
}

impl<'a> DimensionType<'a> {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                (
                    "has_skylight".into(),
                    NbtTag::Byte(if self.has_skylight { 1 } else { 0 }),
                ),
                (
                    "has_ceiling".into(),
                    NbtTag::Byte(if self.has_ceiling { 1 } else { 0 }),
                ),
                (
                    "ultrawarm".into(),
                    NbtTag::Byte(if self.ultrawarm { 1 } else { 0 }),
                ),
                (
                    "natural".into(),
                    NbtTag::Byte(if self.natural { 1 } else { 0 }),
                ),
                (
                    "coordinate_scale".into(),
                    NbtTag::Double(self.coordinate_scale as f64),
                ),
                (
                    "bed_works".into(),
                    NbtTag::Byte(if self.bed_works { 1 } else { 0 }),
                ),
                (
                    "respawn_anchor_works".into(),
                    NbtTag::Byte(if self.respawn_anchor_works { 1 } else { 0 }),
                ),
                ("min_y".into(), NbtTag::Int(self.min_y)),
                ("height".into(), NbtTag::Int(self.height)),
                ("logical_height".into(), NbtTag::Int(self.logical_height)),
                (
                    "infiniburn".into(),
                    NbtTag::String(self.infiniburn.into()),
                ),
                ("effects".into(), self.effects.to_nbt()),
                ("ambient_light".into(), NbtTag::Float(self.ambient_light)),
                (
                    "piglin_safe".into(),
                    NbtTag::Byte(if self.piglin_safe { 1 } else { 0 }),
                ),
                (
                    "has_raids".into(),
                    NbtTag::Byte(if self.has_raids { 1 } else { 0 }),
                ),
                (
                    "monster_spawn_light_level".into(),
                    NbtTag::Int(self.monster_spawn_light_level),
                ),
                (
                    "monster_spawn_block_light_limit".into(),
                    NbtTag::Int(self.monster_spawn_block_light_limit),
                ),
            ]),
        )
    }
}
