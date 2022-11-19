use bevy::prelude::*;
use big_space::{FloatingOrigin, GridPosition};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(big_space::FloatingOriginPlugin::<i32>::default())
        .add_startup_system(setup)
        .add_system(rotator_system)
        .run()
}

/// this component indicates what entities should rotate
#[derive(Component)]
struct Rotator;

/// rotates the parent, which will result in the child also rotating
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in &mut query {
        transform.rotate_x(3.0 * time.delta_seconds());
    }
}

/// set up a simple scene with a "parent" cube and a "child" cube
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });

    // parent cube
    commands
        .spawn(PbrBundle {
            mesh: cube_handle.clone(),
            material: cube_material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1000000000000000.0),
            ..default()
        })
        .insert(GridPosition::<i32>::default())
        .insert(Rotator)
        .with_children(|parent| {
            // child cube
            parent.spawn(PbrBundle {
                mesh: cube_handle,
                material: cube_material_handle,
                transform: Transform::from_xyz(0.0, 0.0, 3.0),
                ..default()
            });
        });
    // light
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(4.0, 5.0, 1000000000000004.0),
            ..default()
        })
        .insert(GridPosition::<i32>::default());
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(5.0, 10.0, 1000000000000000.0)
                .looking_at(Vec3::new(0.0, 0.0, 1000000000000000.0), Vec3::Y),
            ..default()
        })
        .insert(GridPosition::<i32>::default())
        .insert(FloatingOrigin);
}
