mod setup;
mod trace;
mod shaders;
mod animate;

mod plugin;
pub use self::plugin::TracersPlugin;

mod bundle;
pub use self::bundle::{Tracer, TracerBundle, TracerData, TracerTimer};

pub struct TracerPipeline(bevy::asset::Handle<bevy::render::pipeline::PipelineDescriptor>);
