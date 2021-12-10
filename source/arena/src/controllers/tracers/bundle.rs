use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_graph::base::MainPass;
use bevy::render::renderer::RenderResources;

pub struct Tracer {
    pub length: u64,
    pub color: Color,
    pub point: Vec3,
    pub target: Vec3,
}

#[derive(Default)]
pub struct TracerTimer(pub Timer);

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
pub struct TracerData {
    pub color: Color,
}

#[derive(Bundle, Default)]
pub struct TracerBundle {
    pub mesh: Handle<Mesh>,
    pub draw: Draw,
    pub data: TracerData,
    pub time: TracerTimer,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub main_pass: MainPass,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
