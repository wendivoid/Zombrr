use bevy::{
    prelude::*,
    render::{
        pipeline::{FrontFace, PipelineDescriptor},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        shader::{ShaderStage, ShaderStages},
    },
};

pub fn setup(
    mut commands: Commands,
    mut render_graph: ResMut<RenderGraph>,
    mut shaders: ResMut<Assets<Shader>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
) {
    let mut pipeline_descriptor = PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("../shaders/sky.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("../shaders/sky.frag"),
        ))),
    });
    pipeline_descriptor.primitive.front_face = FrontFace::Cw;
    commands.insert_resource(crate::PhysicalSkyPipeline(
        pipelines.set(crate::PIPELINE_HANDLE, pipeline_descriptor),
    ));
    // Add an AssetRenderResourcesNode to our Render Graph. This will bind
    // PhysicalSkyMaterial resources to our shader
    render_graph.add_system_node(
        crate::RENDER_NODE,
        AssetRenderResourcesNode::<crate::SkyMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new PHYSICAL_SKY_RENDER_NODE node
    // to the main pass node. This ensures PHYSICAL_SKY_RENDER_NODE runs before
    // the main pass.
    render_graph
        .add_node_edge(crate::RENDER_NODE, base::node::MAIN_PASS)
        .unwrap();
}
