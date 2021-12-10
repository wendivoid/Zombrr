use bevy::prelude::*;
use bevy::render::pipeline::{RenderPipeline, RenderPipelines};
use bevy::render::render_graph::base::MainPass;

#[derive(Bundle)]
pub struct SkyBundle {
    pub sun: crate::Sun,
    pub mesh: Handle<Mesh>,
    pub draw: Draw,
    pub visible: Visible,
    pub material: Handle<crate::SkyMaterial>,
    pub render_pipelines: RenderPipelines,
    pub main_pass: MainPass,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for SkyBundle {
    fn default() -> SkyBundle {
        SkyBundle {
            sun: Default::default(),
            mesh: Default::default(),
            draw: Default::default(),
            visible: Default::default(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                crate::PIPELINE_HANDLE.typed(),
            )]),
            material: Default::default(),
            main_pass: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
