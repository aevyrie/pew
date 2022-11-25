pub mod body;
pub mod camera;
pub mod post_processing;
pub mod sunlight;

use bevy::{
    pbr::{
        wireframe::{WireframeConfig, WireframePlugin},
        PbrPlugin,
    },
    prelude::*,
    render::settings::{WgpuFeatures, WgpuSettings},
};

use big_space::{FloatingOrigin, FloatingOriginSettings, GridCell};
use body::{Atmosphere, Body};
use camera::CameraController;
use sunlight::Sunlight;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .disable::<TransformPlugin>()
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
                .set(PbrPlugin {
                    prepass_enabled: true,
                })
                .set(WindowPlugin {
                    #[cfg(not(target_arch = "wasm32"))]
                    window: WindowDescriptor {
                        // cursor_visible: false,
                        // cursor_grab_mode: bevy::window::CursorGrabMode::Locked,
                        mode: bevy::window::WindowMode::Fullscreen,
                        ..default()
                    },
                    #[cfg(target_arch = "wasm32")]
                    window: WindowDescriptor {
                        fit_canvas_to_parent: true,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(big_space::FloatingOriginPlugin::<i128> {
            settings: FloatingOriginSettings::new(10_000.0, 100.0),
            ..default()
        })
        .add_plugin(big_space::debug::FloatingOriginDebugPlugin::<i128>::default())
        .add_plugin(post_processing::PostProcessingPlugin)
        .add_plugin(body::BodyPlugin)
        .add_plugin(sunlight::SunlightPlugin)
        .add_plugin(camera::CameraControllerPlugin)
        .insert_resource(Msaa {
            #[cfg(target_arch = "wasm32")]
            samples: 1,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        sunlight::SunlightCamera,
        Camera3dBundle {
            projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
                // fov: 1.5,
                ..default()
            }),
            transform: Transform::from_xyz(5.0, 5.0, 226.5),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        UiCameraConfig { show_ui: false },
        GridCell::<i128>::new(0, 0, 999_370),
        FloatingOrigin,
        CameraController::new(299_792_458.0 * 50_000_000.0, 100.0),
        #[cfg(not(target_arch = "wasm32"))]
        bevy::core_pipeline::bloom::BloomSettings {
            intensity: 0.1,
            ..default()
        },
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 10.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 220.5),
                rotation: Quat::from_euler(EulerRot::XYZ, 0.5, 0.5, 1.0),
                ..default()
            },
            ..default()
        },
        GridCell::<i128>::new(0, 0, 999_370),
    ));

    commands.spawn((
        SpatialBundle::default(),
        Sunlight {
            illuminance: 100000.0,
            color: Color::WHITE,
        },
        Body {
            radius: 250_000_000f32,
        },
        materials.add(StandardMaterial {
            emissive: Color::rgb_linear(3.0, 2.0, 2.0),
            base_color: Color::rgb_linear(3.0, 2.0, 2.0),
            unlit: true,
            ..Default::default()
        }),
        GridCell::<i128>::default(),
    ));

    commands.spawn((
        SpatialBundle::default(),
        Body {
            radius: 6_300_000f32,
        },
        Atmosphere {
            color: Color::RED,
            radius: 6_800_000f32,
        },
        materials.add(StandardMaterial {
            base_color: Color::MIDNIGHT_BLUE,
            perceptual_roughness: 0.9,
            ..Default::default()
        }),
        GridCell::<i128>::new(0, 0, 1_000_000),
    ));
}
