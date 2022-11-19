pub mod camera;
pub mod starfield;
pub mod atmosphere;

use bevy::{prelude::*, sprite::Material2dPlugin};

use camera::CameraController;
use floating_origin::{FloatingOrigin, FloatingOriginSettings, GridPosition};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    #[cfg(not(target_arch = "wasm32"))]
                    window: WindowDescriptor {
                        cursor_visible: false,
                        cursor_grab_mode: bevy::window::CursorGrabMode::Locked,
                        mode: bevy::window::WindowMode::Fullscreen,
                        ..default()
                    },
                    ..default()
                })
                .disable::<bevy::transform::TransformPlugin>(),
        )
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(floating_origin::FloatingOriginPlugin::<i128>::default())
    .add_plugin(Material2dPlugin::<atmosphere::PostProcessingMaterial>::default())
        .add_plugin(camera::CameraControllerPlugin)
        .add_plugin(starfield::StarfieldMaterialPlugin)
        .insert_resource(Msaa {
            #[cfg(target_arch = "wasm32")]
            samples: 1,
            ..default()
        })
        .insert_resource(FloatingOriginSettings::new(10_000.0, 100.0))
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .add_startup_system(starfield::setup)
        .run()
}

/// set up a simple scene with a "parent" cube and a "child" cube
fn setup(
    // settings: Res<FloatingOriginSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    let camera = &mut commands.spawn((
        Camera3dBundle {
            projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
                // fov: 1.5,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 450.0),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        GridPosition::<i128> {
            x: 0,
            y: 0,
            z: 999_370,
        },
        FloatingOrigin,
        CameraController::new(299_792_458.0 * 5_000_000.0, 100.0),
    ));
    #[cfg(not(target_arch = "wasm32"))]
    camera.insert(bevy::core_pipeline::bloom::BloomSettings {
        threshold: 2.0,
        knee: 0.1,
        scale: 1.0,
        intensity: 0.1,
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 452.0),
            ..default()
        },
        GridPosition::<i128> {
            x: 0,
            y: 0,
            z: 999_370,
        },
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::default().looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
            ..default()
        },
        GridPosition::<i128>::default(),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 250_000_000.0,
                subdivisions: 16,
            })),
            material: materials.add(StandardMaterial {
                emissive: Color::rgb_linear(16.0, 15.0, 8.0),
                ..Default::default()
            }),
            ..default()
        },
        GridPosition::<i128>::default(),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 6_300_000.0,
                subdivisions: 42,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::MIDNIGHT_BLUE,
                perceptual_roughness: 0.9,
                ..Default::default()
            }),
            ..default()
        },
        GridPosition::<i128> {
            x: 0,
            y: 0,
            z: 1_000_000,
        },
    ));
}
