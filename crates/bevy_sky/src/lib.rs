mod material;
pub use self::material::SkyMaterial;

mod plugin;
pub use self::plugin::SkyPlugin;

mod entities;
pub use self::entities::*;

mod systems;

pub const DEFAULT_SUN_DISTANCE: f32 = 400000.0;
pub const SETUP_SYSTEM: &str = "sky_setup";
pub const RENDER_NODE: &str = "sky";

use bevy::asset::{Handle, HandleUntyped};
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::PipelineDescriptor;

pub const PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 9346485865180481512);

pub struct PhysicalSkyPipeline(pub Handle<PipelineDescriptor>);
