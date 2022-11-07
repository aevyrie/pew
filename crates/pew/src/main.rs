pub mod camera;

use bevy::{math::DVec3, prelude::*};
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
    settings: Res<FloatingOriginSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });

    commands
        .spawn_bundle(Camera3dBundle {
            projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
                fov: 1.5,
                ..default()
            }),
            transform: Transform::from_xyz(-8.0, 7.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(GridPosition::<i64>::default());

    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
                // fov: 1.5,
                ..default()
            }),
            camera: Camera {
                priority: 100,
                is_active: false,
                ..default()
            },
            ..default()
        })
        .insert(GridPosition::<i64>::default())
        .insert(FloatingOriginCamera)
        .insert(camera::CameraController)
        .with_children(|parent| {
            // child cube
            parent.spawn_bundle(PbrBundle {
                mesh: cube_handle.clone(),
                material: cube_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 3.0),
                ..default()
            });
        });

    commands
        .spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::default().looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(GridPosition::<i64> {
            x: 0,
            y: 0,
            z: 140_000_000,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 700_000_000.0,
                    subdivisions: 3,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    unlit: true,
                    ..Default::default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });
        });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 250_000_000_000.0,
                subdivisions: 3,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                ..Default::default()
            }),
            ..default()
        })
        .insert(GridPosition::<i64> {
            x: 0,
            y: 200_000_000_000_000,
            z: 0,
        });

    // // parent cube
    // commands
    //     .spawn_bundle(SceneBundle {
    //         scene: asset_server.load("models/earth/Earth_1_12756.gltf#Scene0"),
    //         transform: Transform::from_xyz(0.0, 0.0, -6_000_100.0)
    //             .with_scale(Vec3::splat(12_000.0))
    //             .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
    //         ..default()
    //     })
    //     .insert(GridPosition::<i64>::default());

    let radius = 58_000_000.0f64; // meters

    let model_half_extent = 500.0; // meters
    let scale = Vec3::splat(radius as f32 / model_half_extent);

    let position = DVec3::new(0.0, 0.0, -(radius + 100.0));
    let (grid, transform) = settings.precise_position::<i64>(position);
    let transform = transform.with_scale(scale);

    commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("models/saturn/Saturn_1_120536.gltf#Scene0"),
            transform,
            //.with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ..default()
        })
        .insert(grid);
}
