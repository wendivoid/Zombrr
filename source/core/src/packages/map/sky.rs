use serde::{ Serialize, Deserialize };

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Sky {
    #[serde(default = "default_lat")]
    pub latitude: f32,
    #[serde(default = "default_long")]
    pub longitude: f32,
    #[serde(default = "default_seconds_per_second")]
    pub day_length: f32,
    #[serde(default = "default_sky_size")]
    pub sky_size: f32,
    #[serde(default)]
    pub active: bool,
    #[serde(default = "default_distance")]
    pub distance: f32,
    #[serde(default = "default_preset")]
    pub preset: SkyPreset
}

fn default_lat() -> f32 { 59.33258 }
fn default_long() -> f32 { 59.33258 }
fn default_distance() -> f32 { 400000.0 }
fn default_sky_size() -> f32 { 1500.0 }
fn default_seconds_per_second() -> f32 { 1000.0 }
fn default_preset() -> SkyPreset { Default::default() }

impl Default for Sky {
    fn default() -> Sky {
        Sky {
            latitude: 59.33258,
            longitude: 18.0649,
            day_length: 1000.0,
            sky_size: 1500.0,
            active: false,
            distance: 400000.0,
            preset: Default::default()
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SkyPreset {
    BloodSky,
    AlientDay,
    StellarDawn,
    RedSunset,
    BlueDusk,
    PurpleDusk,
    Custom {
        mie_k_coefficient: [f32 ; 4],
        primaries: [f32 ; 4],
        sun_position: [f32 ; 4],
        depolarization_factor: f32,
        luminance: f32,
        mie_coefficient: f32,
        mie_directional_g: f32,
        mie_v: f32,
        mie_zenith_length: f32,
        num_molecules: f32,
        rayleigh: f32,
        rayleigh_zenith_length: f32,
        refractive_index: f32,
        sun_angular_diameter_degrees: f32,
        sun_intensity_factor: f32,
        sun_intensity_falloff_steepness: f32,
        tonemap_weighting: f32,
        turbidity: f32,
    }
}

impl Default for SkyPreset {
    fn default() -> SkyPreset {
        SkyPreset::AlientDay
    }
}
