use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

use crate::network::types::TemperatureModifier;

pub struct Biome {
    pub name: String,
    pub has_precipitation: bool,
    pub temperature: f32,
    pub temperature_modifier: TemperatureModifier,
    pub downfall: f32,
    pub effects: BiomeEffects,
}

impl Biome {
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                (
                    "has_precipitation".into(),
                    NbtTag::Byte(self.has_precipitation as i8),
                ),
                ("temperature".into(), NbtTag::Float(self.temperature)),
                (
                    "temperature_modifier".into(),
                    self.temperature_modifier.to_nbt(),
                ),
                ("downfall".into(), NbtTag::Float(self.downfall)),
                ("effects".into(), self.effects.to_nbt()),
            ]),
        )
    }
}

pub struct BiomeEffects {
    pub fog_color: i32,
    pub water_color: i32,
    pub water_fog_color: i32,
    pub sky_color: i32,
    pub foliage_color: Option<i32>,
    pub grass_color: Option<i32>,
    pub grass_color_modifier: Option<String>,
    pub particle: Option<Particle>,
    pub ambient_sound: Option<AmbientSound>,
    pub mood_sound: Option<MoodSound>,
    pub additions_sound: Option<AdditionsSound>,
    pub music: Option<Music>,
}

impl BiomeEffects {
    pub fn to_nbt(&self) -> NbtTag {
        NbtTag::Compound(NbtCompound::from_values(vec![
            ("fog_color".into(), NbtTag::Int(self.fog_color)),
            ("water_color".into(), NbtTag::Int(self.water_color)),
            ("water_fog_color".into(), NbtTag::Int(self.water_fog_color)),
            ("sky_color".into(), NbtTag::Int(self.sky_color)),
        ]))
    }
}

pub struct Particle {
    pub options: ParticleOptions,
    pub probability: f32,
}

pub struct ParticleOptions {
    pub particle_type: String, // TODO: More things
}

pub struct AmbientSound {
    pub sound_id: String,
    pub range: Option<f32>,
}

pub struct MoodSound {
    pub sound: String,
    pub tick_delay: i32,
    pub block_search_extent: i32,
    pub offset: f64,
}

pub struct AdditionsSound {
    pub sound: String,
    pub tick_chance: f64,
}

pub struct Music {
    pub sound: String,
    pub min_delay: i32,
    pub max_delay: i32,
    pub replace_current_music: bool,
}
