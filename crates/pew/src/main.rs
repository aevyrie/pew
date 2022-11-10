pub mod camera;
pub mod starfield;

use bevy::prelude::*;

use floating_origin::{FloatingOriginCamera, FloatingOriginSettings, GridPosition};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            cursor_visible: false,
            // cursor_locked: true,
            mode: bevy::window::WindowMode::Fullscreen,
            ..default()
        })
        .add_plugins_with(DefaultPlugins, |group| {
            group.disable::<bevy::transform::TransformPlugin>() // Disable built-in transform plugin
        })
        .add_plugin(floating_origin::FloatingOriginPlugin::<i64>::default())
        .add_plugin(camera::CameraControllerPlugin)
        .add_plugin(starfield::StarfieldMaterialPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(InspectorPlugin::<FloatingOriginSettings>::new())
        .insert_resource(Msaa { samples: 4 })
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
    commands
        .spawn_bundle(Camera3dBundle {
            projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
                // fov: 1.5,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 258.0),
            ..default()
        })
        .insert(GridPosition::<i64> {
            x: 0,
            y: 0,
            z: 999_370,
        })
        .insert(FloatingOriginCamera)
        .insert(camera::CameraController::new(
            299_792_458.0 * 5_000_000.0,
            100.0,
        ));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 259.0),
            ..default()
        })
        .insert(GridPosition::<i64> {
            x: 0,
            y: 0,
            z: 999_370,
        });

    commands
        .spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::default().looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
            ..default()
        })
        .insert(GridPosition::<i64>::default());

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 250_000_000.0,
                subdivisions: 16,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb_linear(1.0, 0.95, 0.65),
                unlit: true,
                ..Default::default()
            }),
            ..default()
        })
        .insert(GridPosition::<i64>::default());

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 6_300_000.0,
                subdivisions: 42,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::MIDNIGHT_BLUE,
                ..Default::default()
            }),
            ..default()
        })
        .insert(GridPosition::<i64> {
            x: 0,
            y: 0,
            z: 1_000_000,
        });
}
