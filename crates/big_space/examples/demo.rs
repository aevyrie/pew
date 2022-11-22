use bevy::prelude::*;
use big_space::{FloatingOrigin, FloatingOriginSettings, GridCell};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<TransformPlugin>())
        .add_plugin(big_space::FloatingOriginPlugin::<i64>::default())
        .add_plugin(big_space::debug::FloatingOriginDebugPlugin::<i64>::default())
        .insert_resource(FloatingOriginSettings::new(1.0, 0.01))
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .add_system(rotator_system)
        .run()
}

#[derive(Component)]
struct Rotator<const N: usize>;

/// rotates the parent, which will result in the child also rotating
fn rotator_system(
    time: Res<Time>,
    mut q: ParamSet<(
        Query<&mut Transform, With<Rotator<1>>>,
        Query<&mut Transform, With<Rotator<2>>>,
        Query<&mut Transform, With<Rotator<3>>>,
    )>,
) {
    let delta_translation = |offset: f32| -> Vec3 {
        let t_1 = time.elapsed_seconds() + offset;
        let dt = time.delta_seconds();
        let t_0 = t_1 - dt;
        let pos =
            |t: f32| -> Vec3 { Vec3::new(t.cos() * 2.0, t.sin() * 2.0, (t * 1.3).sin() * 2.0) };
        let p0 = pos(t_0);
        let p1 = pos(t_1);
        let dp = p1 - p0;
        dp
    };

    q.p0().single_mut().translation += delta_translation(20.0);
    q.p1().single_mut().translation += delta_translation(251.0);
    q.p2().single_mut().translation += delta_translation(812.0);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 0.5 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });

    // cube
    commands
        .spawn(PbrBundle {
            mesh: cube_handle.clone(),
            material: cube_material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        })
        .insert(GridCell::<i64>::default())
        .insert(Rotator::<1>);
    commands
        .spawn(PbrBundle {
            mesh: cube_handle.clone(),
            material: cube_material_handle.clone(),
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        })
        .insert(GridCell::<i64>::default())
        .insert(Rotator::<2>);
    commands
        .spawn(PbrBundle {
            mesh: cube_handle.clone(),
            material: cube_material_handle.clone(),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(GridCell::<i64>::default())
        .insert(Rotator::<3>);
    // light
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(GridCell::<i64>::default());
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 8.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(GridCell::<i64>::default())
        .insert(FloatingOrigin);
}
