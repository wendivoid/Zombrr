use zombrr_core::{ArenaOptions, ZombrrPackages};
use zombrr_core::packages::{SkyPreset, MapData};
use bevy::prelude::*;
use bevy::pbr::AmbientLight;
use chrono::{TimeZone, Utc};

use bevy_sky::{SkyMaterial, SkyBundle, Sun};

use crate::arena::ArenaMapData;

pub fn spawn_arena(
    mut commands: Commands,
    options: Res<ArenaOptions>,
    packages: Res<ZombrrPackages>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut resources: ResMut<crate::arena::ArenaResources>,
    mut sky_materials: ResMut<Assets<SkyMaterial>>,
) {
    if let Some(map) = packages.get_map(&options.map) {
        debug!(
            "Spawning Map `{}`\n\t-> Name = {}\n\t-> Path: {:?}\n\t-> Ambient Light = {:?}\n\t-> Skybox = {:?}",
            map.name, map.name, map.path, map.meta.ambient_light, map.meta.sky
        );
        match &map.meta.map {
            MapData::Gltf { path } => {
                let mut map_path = map.path.clone();
                map_path.push(path);
                let asset_path = format!("{}#Scene0", map_path.to_str().unwrap());
                let scene = asset_server.load(asset_path.as_str());
                let instance_id = scene_spawner.spawn(scene.clone());
                resources.map = Some(ArenaMapData {
                    name: map.name.clone(),
                    scene,
                    instance_id,
                    loaded: false
                });
            }
        }
        commands.insert_resource(AmbientLight {
            color: crate::utils::zombrr_color_to_bevy(&map.meta.ambient_light.color),
            brightness: map.meta.ambient_light.brightness
        });
        commands.spawn_bundle(SkyBundle {
            sun: Sun {
                latitude: map.meta.sky.latitude as f64,
                longitude: map.meta.sky.longitude as f64,
                simulation_seconds_per_second: 24.0 * 60.0 * 60.0 / map.meta.sky.day_length as f64,
                active: map.meta.sky.active,
                distance: map.meta.sky.distance,
                now: Utc.ymd(2021, 03, 01).and_hms(7, 0, 0)
            },
            mesh: meshes.add(Mesh::from(shape::Cube { size: map.meta.sky.sky_size })),
            material: sky_materials.add(preset_to_material(&map.meta.sky.preset)),
            ..Default::default()
        })
        .insert(Name::new("Arena Map SkyBox"))
        .insert(super::ArenaMapSkyBox);
    } else {
        error!("Map not found: {:?}", options.map);
    }
}

fn preset_to_material(preset: &SkyPreset) -> SkyMaterial {
    match preset {
        SkyPreset::BloodSky => SkyMaterial::blood_sky(),
        SkyPreset::AlientDay => SkyMaterial::alien_day(),
        SkyPreset::StellarDawn => SkyMaterial::stellar_dawn(),
        SkyPreset::RedSunset => SkyMaterial::red_sunset(),
        SkyPreset::BlueDusk => SkyMaterial::blue_dusk(),
        SkyPreset::PurpleDusk => SkyMaterial::purple_dusk(),
        SkyPreset::Custom {
            mie_k_coefficient,
            primaries,
            sun_position,
            depolarization_factor,
            luminance,
            mie_coefficient,
            mie_directional_g,
            mie_v,
            mie_zenith_length,
            num_molecules,
            rayleigh,
            rayleigh_zenith_length,
            refractive_index,
            sun_angular_diameter_degrees,
            sun_intensity_factor,
            sun_intensity_falloff_steepness,
            tonemap_weighting,
            turbidity,
        } => {
            SkyMaterial {
                mie_k_coefficient: Vec4::from(*mie_k_coefficient),
                primaries: Vec4::from(*primaries),
                sun_position: Vec4::from(*sun_position),
                depolarization_factor: *depolarization_factor,
                luminance: *luminance,
                mie_coefficient: *mie_coefficient,
                mie_directional_g: *mie_directional_g,
                mie_v: *mie_v,
                mie_zenith_length: *mie_zenith_length,
                num_molecules: *num_molecules,
                rayleigh: *rayleigh,
                rayleigh_zenith_length: *rayleigh_zenith_length,
                refractive_index: *refractive_index,
                sun_angular_diameter_degrees: *sun_angular_diameter_degrees,
                sun_intensity_factor: *sun_intensity_factor,
                sun_intensity_falloff_steepness: *sun_intensity_falloff_steepness,
                tonemap_weighting: *tonemap_weighting,
                turbidity: *turbidity,
            }
        }
    }
}
