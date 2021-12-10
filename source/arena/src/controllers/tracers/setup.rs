use bevy::prelude::*;
use bevy::render::pipeline::PipelineDescriptor;
use bevy::render::render_graph::{base, RenderGraph, RenderResourcesNode};
use bevy::render::shader::{ShaderStage, ShaderStages};

use super::TracerData;

pub fn setup_tracer_graphics(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Create a new shader pipeline.
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            super::shaders::VERTEX_SHADER,
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            super::shaders::FRAGMENT_SHADER,
        ))),
    }));

    render_graph.add_system_node("tracer_data", RenderResourcesNode::<TracerData>::new(true));

    render_graph
        .add_node_edge("tracer_data", base::node::MAIN_PASS)
        .unwrap();
    commands.insert_resource(super::TracerPipeline(pipeline_handle));
}
