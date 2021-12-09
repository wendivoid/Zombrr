use bevy::prelude::*;
use std::time::Duration;
use bevy::render::pipeline::RenderPipeline;
use bevy::render::pipeline::PrimitiveTopology;

use super::*;

pub fn spawn_trace_bundles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    pipeline: Res<super::TracerPipeline>,
    query: Query<(Entity, &Tracer), Added<Tracer>>,
) {
    for (entity, tracer) in query.iter() {
        commands.entity(entity)
            .insert_bundle(TracerBundle {
                mesh: meshes.add(generate_mesh(tracer.point, tracer.target)),
                time: TracerTimer(Timer::new(Duration::from_millis(tracer.length), false)),
                data: TracerData {
                    color: tracer.color
                },
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    pipeline.0.clone(),
                )]),
                ..Default::default()
            })
            .remove::<Tracer>();
    }
}

fn generate_mesh(point: Vec3, other: Vec3) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    let positions = vec![
        [point.x, point.y, point.z],
        [other.x, other.y, other.z]
    ];
    let uvs = vec![
        [0.0, 0.0],
        [1.0, 0.0]
    ];
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}
