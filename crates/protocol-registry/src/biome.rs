use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

use crate::network::types::TemperatureModifier;

pub struct Biome<'a> {
    pub name: &'a str,
    pub has_precipitation: bool,
    pub temperature: f32,
    pub temperature_modifier: TemperatureModifier,
    pub downfall: f32,
    pub effects: BiomeEffects<'a>,
}

impl<'a> Biome<'a> {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                (
                    "has_precipitation".into(),
                    NbtTag::Byte(self.has_precipitation as i8),
                ),
                ("temperature".into(), NbtTag::Float(self.temperature)),
                ("downfall".into(), NbtTag::Float(self.downfall)),
                ("effects".into(), self.effects.to_nbt()),
            ]),
        )
    }
}

pub struct BiomeEffects<'a> {
    pub fog_color: i32,
    pub water_color: i32,
    pub water_fog_color: i32,
    pub sky_color: i32,
    pub foliage_color: Option<i32>,
    pub grass_color: Option<i32>,
    pub grass_color_modifier: Option<&'a str>,
    pub particle: Option<Particle<'a>>,
    pub ambient_sound: Option<AmbientSound<'a>>,
    pub mood_sound: Option<MoodSound<'a>>,
    pub additions_sound: Option<AdditionsSound<'a>>,
    pub music: Option<Music<'a>>,
}

impl<'a> BiomeEffects<'a> {
    pub fn to_nbt(&self) -> NbtTag {
        NbtTag::Compound(NbtCompound::from_values(vec![
            ("fog_color".into(), NbtTag::Int(self.fog_color)),
            ("water_color".into(), NbtTag::Int(self.water_color)),
            ("water_fog_color".into(), NbtTag::Int(self.water_fog_color)),
            ("sky_color".into(), NbtTag::Int(self.sky_color)),
        ]))
    }
}

pub struct Particle<'a> {
    pub options: ParticleOptions<'a>,
    pub probability: f32,
}

pub struct ParticleOptions<'a> {
    pub particle_type: &'a str, // TODO: More things
}

pub struct AmbientSound<'a> {
    pub sound_id: &'a str,
    pub range: Option<f32>,
}

pub struct MoodSound<'a> {
    pub sound: &'a str,
    pub tick_delay: i32,
    pub block_search_extent: i32,
    pub offset: f64,
}

pub struct AdditionsSound<'a> {
    pub sound: &'a str,
    pub tick_chance: f64,
}

pub struct Music<'a> {
    pub sound: &'a str,
    pub min_delay: i32,
    pub max_delay: i32,
    pub replace_current_music: bool,
}
