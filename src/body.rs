use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct BodyPlugin;
impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(BodyMesh::setup)
            .add_system(Body::update)
            .add_plugin(MaterialPlugin::<AtmosphereMaterial> {
                prepass_enabled: true,
                ..default()
            })
            .add_system(Atmosphere::update);
    }
}

#[derive(Component, Reflect)]
pub struct Body {
    pub radius: f32,
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
    fn update(
        mut commands: Commands,
        base_body_mesh: Res<BodyMesh>,
        mut changed_bodies: Query<(&Body, &mut Transform), (Changed<Body>, With<Handle<Mesh>>)>,
        mut no_mesh_bodies: Query<
            (Entity, &Body, &mut Transform),
            (Changed<Body>, Without<Handle<Mesh>>),
        >,
    ) {
        for (body, mut transform) in changed_bodies.iter_mut() {
            transform.scale = Vec3::splat(body.radius);
        }
        for (entity, body, mut transform) in no_mesh_bodies.iter_mut() {
            commands.entity(entity).insert(base_body_mesh.0.clone());
            transform.scale = Vec3::splat(body.radius);
        }
    }
}

#[derive(Component, Reflect)]
pub struct Atmosphere {
    pub color: Color,
    pub radius: f32,
}

impl Atmosphere {
    pub fn update(
        mut commands: Commands,
        base_body_mesh: Res<BodyMesh>,
        mut atm_matls: ResMut<Assets<AtmosphereMaterial>>,
        changed_atmospheres: Query<
            (Entity, Option<&Children>, &Atmosphere, &Body),
            Changed<Atmosphere>,
        >,
        mut atmos_children: Query<(&mut Handle<AtmosphereMaterial>, &mut Transform), With<Parent>>,
    ) {
        let mesh = base_body_mesh.0.clone();

        for (entity, children, atmosphere, body) in &changed_atmospheres {
            let scale = atmosphere.radius / body.radius;
            let new_atmos_matl = atm_matls.add(AtmosphereMaterial::from(atmosphere));

            let mut needs_spawn = true;
            for &child in children.iter().flat_map(|c| c.iter()) {
                if let Ok((mut matl, mut transform)) = atmos_children.get_mut(child) {
                    transform.scale = Vec3::splat(scale);
                    *matl = new_atmos_matl.clone();
                    needs_spawn = false;
                    break;
                }
            }

            if needs_spawn {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn((
                        SpatialBundle {
                            transform: Transform::from_scale(Vec3::splat(scale)),
                            ..default()
                        },
                        mesh.clone(),
                        new_atmos_matl,
                    ));
                });
            }
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone, Reflect)]
#[uuid = "cf29696f-7fb3-45ff-afea-43541fdf0ac3"]
pub struct AtmosphereMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    radius: f32,
}

impl From<Atmosphere> for AtmosphereMaterial {
    fn from(a: Atmosphere) -> Self {
        AtmosphereMaterial {
            color: a.color,
            radius: a.radius,
        }
    }
}

impl From<&Atmosphere> for AtmosphereMaterial {
    fn from(a: &Atmosphere) -> Self {
        AtmosphereMaterial {
            color: a.color,
            radius: a.radius,
        }
    }
}

/// Not shown in this example, but if you need to specialize your material, the specialize
/// function will also be used in the prepass
impl Material for AtmosphereMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/atmosphere.wgsl".into()
    }

    // TODO: Make a vertex shader that, when the camera is inside the atmosphere, takes vertices
    // behind the camera and smooshes them to be just in front of the near camera frustum. This will
    // ensure the atmosphere is always drawn on top of things inside the atmosphere because the
    // camera will never actually enter the atmosphere, as the mesh that would be behind it is now
    // pushed in front. Use the specialization feature to change vert shader when the camera is
    // inside that atmosphere?
    //
    // fn vertex_shader() -> ShaderRef { "shaders/atmosphere.wgsl".into() }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn depth_bias(&self) -> f32 {
        1.0
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayout,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
