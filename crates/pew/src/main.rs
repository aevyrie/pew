pub mod camera;

use bevy::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};
use floating_origin::{FloatingOriginCamera, FloatingOriginSettings, GridPosition};

fn main() {
    App::new()
        .add_plugins_with(DefaultPlugins, |group| {
            group.disable::<bevy::transform::TransformPlugin>() // Disable built-in transform plugin
        })
        .add_plugin(floating_origin::FloatingOriginPlugin::<i64>::default())
        .add_plugin(camera::CameraControllerPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<FloatingOriginSettings>::new())
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(FloatingOriginSettings::new(10_000.0, 100.0))
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
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
                fov: 1.5,
                ..default()
            }),
            ..default()
        })
        .insert(GridPosition::<i64> {
            x: 0,
            y: 0,
            z: 1_000_000,
        })
        .insert(FloatingOriginCamera)
        .insert(camera::CameraController);

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_at(-Vec3::ONE, Vec3::Y),
        ..default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 250_000_000.0,
                subdivisions: 16,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
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
                subdivisions: 16,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::CYAN,
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
