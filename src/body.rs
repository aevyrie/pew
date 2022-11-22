use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct BodyPlugin;
impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(BodyMesh::setup)
            .add_system(Body::update_system);
    }
}

#[derive(Component)]
pub struct Body {
    pub radius: usize,
}

#[derive(Resource)]
pub struct BodyMesh(Handle<Mesh>);

impl BodyMesh {
    pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        commands.insert_resource(BodyMesh(
            meshes.add(
                shape::Icosphere {
                    radius: 1.0,
                    subdivisions: 64,
                }
                .try_into()
                .unwrap(),
            ),
        ));
    }
}

impl Body {
    fn update_system(
        mut commands: Commands,
        base_body_mesh: Res<BodyMesh>,
        mut bodies: Query<(&Body, &mut Transform), (Changed<Body>, With<Handle<Mesh>>)>,
        mut no_mesh_bodies: Query<
            (Entity, &Body, &mut Transform),
            (Changed<Body>, Without<Handle<Mesh>>),
        >,
    ) {
        for (body, mut transform) in bodies.iter_mut() {
            transform.scale = Vec3::splat(body.radius as f32);
        }
        for (entity, body, mut transform) in no_mesh_bodies.iter_mut() {
            commands.entity(entity).insert(base_body_mesh.0.clone());
            transform.scale = Vec3::splat(body.radius as f32);
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct AtmosphereMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(1)]
    radius: f32,
}

/// Not shown in this example, but if you need to specialize your material, the specialize
/// function will also be used in the prepass
impl Material for AtmosphereMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/atmosphere.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    // You can override the default shaders used in the prepass if your material does
    // anything not supported by default
    // fn prepass_fragment_shader() -> ShaderRef {
    //     "shaders/custom_material.wgsl".into()
    // }
}
