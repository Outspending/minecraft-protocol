use protocol_registry::{
    armor_trim::{ArmorTrimMaterial, ArmorTrimPattern},
    banner::Banner,
    biome::{Biome, BiomeEffects},
    chat_type::ChatType,
    damage_type::DamageType,
    dimension_type::DimensionType,
    network::types::{DimensionEffects, TemperatureModifier},
    wolf::WolfVariant,
};
use simdnbt::owned::Nbt;

use crate::{
    buffer::{buffer::ByteBuf, varnum::VarInt},
    tcp::client::connection::MinecraftClient,
    v1_21::RegistryDataPacket,
    FromNetwork, ToNetwork,
};

use super::PacketSender;

#[derive(Debug)]
pub struct RegistryEntry {
    pub entry_id: String,
    pub has_data: bool,
    pub data: Nbt,
}

impl ToNetwork for RegistryEntry {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write_string(self.entry_id.clone());
        buf.write_bool(self.has_data);
        if self.has_data {
            self.data.write_unnamed(buf.get_mut());
        }
    }
}

impl FromNetwork for RegistryEntry {
    fn from_network(buf: &mut ByteBuf) -> Self {
        todo!()
    }
}

pub async fn send_registry_packets(client: &mut MinecraftClient) {
    let biome = Biome {
        name: "minecraft:badlands".to_string(),
        has_precipitation: false,
        temperature: 2.0,
        temperature_modifier: TemperatureModifier::None,
        downfall: 0.0,
        effects: BiomeEffects {
            fog_color: 12638463,
            water_color: 4159204,
            water_fog_color: 329011,
            sky_color: 7254527,
            foliage_color: None,
            grass_color: None,
            grass_color_modifier: None,
            particle: None,
            ambient_sound: None,
            mood_sound: None,
            additions_sound: None,
            music: None,
        },
    };

    client
        .send_packet(&RegistryDataPacket {
            registry_id: "minecraft:trim_material".to_string(),
            entry_count: VarInt::from(1),
            entries: vec![RegistryEntry {
                entry_id: biome.name.clone(),
                has_data: true,
                data: biome.to_nbt(),
            }],
        })
        .await;

    let chat_type = ChatType {
        name: "minecraft:chat".to_string(),
        translation_key: "chat.type.text".to_string(),
        style: None,
        parameters: "sender".to_string(),
    };

    client
        .send_packet(&RegistryDataPacket {
            registry_id: "minecraft:chat_type".to_string(),
            entry_count: VarInt::from(1),
            entries: vec![RegistryEntry {
                entry_id: chat_type.name.clone(),
                has_data: true,
                data: chat_type.to_nbt(),
            }],
        })
        .await;

    let trim_pattern = ArmorTrimPattern {
        name: "minecraft:coast".to_string(),
        asset_id: "minecraft:coast".to_string(),
        template_item: "minecraft:coast_armor_trim_smithing_template".to_string(),
        description: "trim_pattern.minecraft.coast".to_string(),
        decal: 0,
    };

    client
        .send_packet(&RegistryDataPacket {
            registry_id: "minecraft:trim_pattern".to_string(),
            entry_count: VarInt::from(1),
            entries: vec![RegistryEntry {
                entry_id: trim_pattern.name.clone(),
                has_data: true,
                data: trim_pattern.to_nbt(),
            }],
        })
        .await;

    let trim_material = ArmorTrimMaterial {
        name: "minecraft:amethyst".to_string(),
        asset_name: "amethyst".to_string(),
        ingredient: "minecraft:amethyst_shard".to_string(),
        item_model_index: 1.0,
        override_armor_materials: None,
        description: "trim_material.minecraft.amethyst".to_string(),
    };

    client
        .send_packet(&RegistryDataPacket {
            registry_id: "minecraft:trim_material".to_string(),
            entry_count: VarInt::from(1),
            entries: vec![RegistryEntry {
                entry_id: trim_material.name.clone(),
                has_data: true,
                data: trim_material.to_nbt(),
            }],
        })
        .await;

    let wolf_variant = WolfVariant {
        name: "minecraft:ashen".to_string(),
        wild_texture: "minecraft:entity/wolf/wolf_ashen".to_string(),
        tamed_texture: "minecraft:entity/wolf/wolf_ashen_tame".to_string(),
        angry_texture: "minecraft:entity/wolf/wolf_ashen_angry".to_string(),
        biomes: "minecraft:snowy_taiga".to_string(),
    };

    client
        .send_packet(&RegistryDataPacket {
            registry_id: "minecraft:wolf_variant".to_string(),
            entry_count: VarInt::from(1),
            entries: vec![RegistryEntry {
                entry_id: wolf_variant.name.clone(),
                has_data: true,
                data: wolf_variant.to_nbt(),
            }],
        })
        .await;

    let dimension_type = DimensionType {
        name: "minecraft:overworld".to_string(),
        piglin_safe: false,
        natural: true,
        ambient_light: 0.0,
        monster_spawn_block_light_limit: 0,
        infiniburn: "#minecraft:infiniburn_overworld".to_string(),
        respawn_anchor_works: false,
        has_skylight: true,
        bed_works: true,
        effects: DimensionEffects::Overworld,
        has_raids: true,
        logical_height: 384,
        coordinate_scale: 1.0,
        monster_spawn_light_level: 0,
        min_y: -64,
        ultrawarm: false,
        has_ceiling: false,
        height: 384,

        fixed_time: None,
    };

    client
        .send_packet(&RegistryDataPacket {
            registry_id: "minecraft:dimension_type".to_string(),
            entry_count: VarInt::from(1),
            entries: vec![RegistryEntry {
                entry_id: dimension_type.name.clone(),
                has_data: true,
                data: dimension_type.to_nbt(),
            }],
        })
        .await;

    let damage_type = DamageType {
        name: "minecraft:arrow".to_string(),
        message_id: "arrow".to_string(),
        exhaustion: 0.1,
        scaling: "when_caused_by_living_non_player".to_string(),
        effects: None,
        death_message_type: None,
    };

    client
        .send_packet(&RegistryDataPacket {
            registry_id: "minecraft:damage_type".to_string(),
            entry_count: VarInt::from(1),
            entries: vec![RegistryEntry {
                entry_id: damage_type.name.clone(),
                has_data: true,
                data: damage_type.to_nbt(),
            }],
        })
        .await;

    let banner_pattern = Banner {
        name: "minecraft:base".to_string(),
        translation_key: "block.minecraft.banner.base".to_string(),
        asset_id: "minecraft:base".to_string(),
    };

    client
        .send_packet(&RegistryDataPacket {
            registry_id: "minecraft:banner_pattern".to_string(),
            entry_count: VarInt::from(1),
            entries: vec![RegistryEntry {
                entry_id: banner_pattern.name.clone(),
                has_data: true,
                data: banner_pattern.to_nbt(),
            }],
        })
        .await;
}
