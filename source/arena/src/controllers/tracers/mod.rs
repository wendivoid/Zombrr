mod animate;
mod setup;
mod shaders;
mod trace;

mod plugin;
pub use self::plugin::TracersPlugin;

mod bundle;
pub use self::bundle::{Tracer, TracerBundle, TracerData, TracerTimer};

pub struct TracerPipeline(bevy::asset::Handle<bevy::render::pipeline::PipelineDescriptor>);
