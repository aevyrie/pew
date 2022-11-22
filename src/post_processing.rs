use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};
use big_space::FloatingOrigin;

pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_render_target)
            .add_system(update_post_process_image_size)
            .add_plugin(Material2dPlugin::<PostProcessMaterial>::default());
    }
}

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct PostProcessMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
}

impl Material2d for PostProcessMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material_chromatic_aberration.wgsl".into()
    }
}

pub fn setup_render_target(
    // settings: Res<FloatingOriginSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut post_processing_materials: ResMut<Assets<PostProcessMaterial>>,
    mut main_camera: Query<&mut Camera, With<FloatingOrigin>>,
) {
    // This is the texture that will be rendered to.
    let image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d::default(),
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    let image_handle = images.add(image);

    main_camera.single_mut().target = RenderTarget::Image(image_handle.clone());

    // This specifies the layer used for the post processing camera, which will be attached to the
    // post processing camera and 2d quad.
    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0))));
    let material_handle = post_processing_materials.add(PostProcessMaterial {
        source_image: image_handle, // The texture being rendered to
    });

    // Post processing 2d quad, with material using the render texture done by the main camera, with
    // a custom shader.
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..default()
            },
            ..default()
        },
        post_processing_pass_layer,
    ));

    // The post-processing pass camera.
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // renders after the first main camera which has default value: 0.
                priority: 1,
                ..default()
            },
            ..Camera2dBundle::default()
        },
        post_processing_pass_layer,
    ));
}

pub fn update_post_process_image_size(
    windows: Res<Windows>,
    mut matl_handle: Query<(&Handle<PostProcessMaterial>, &mut Mesh2dHandle)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<PostProcessMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    if !windows.is_changed() {
        return;
    }

    let window = windows.primary();
    let size = Extent3d {
        width: window.physical_width(),
        height: window.physical_height(),
        ..default()
    };

    let matl = post_processing_materials
        .get_mut(matl_handle.single().0)
        .unwrap();

    let image = images.get_mut(&matl.source_image).unwrap();

    image.texture_descriptor.size = size;
    image.resize(size);

    matl_handle.single_mut().1 .0 = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        size.width as f32,
        size.height as f32,
    ))));
}
