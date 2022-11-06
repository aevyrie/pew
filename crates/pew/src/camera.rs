use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct CameraControllerPlugin;
impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_controller);
    }
}

#[derive(Component)]
pub struct CameraController;

pub fn camera_controller(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut mouse: EventReader<MouseMotion>,
    mut camera: Query<&mut Transform, With<CameraController>>,
) {
    let mut camera_transform = camera.single_mut();
    let distance = time.delta_seconds() * 100.0;
    let mut translation = Vec3::ZERO;
    if keyboard.pressed(KeyCode::W) {
        translation += camera_transform.forward() * distance;
    }
    if keyboard.pressed(KeyCode::A) {
        translation += camera_transform.left() * distance;
    }
    if keyboard.pressed(KeyCode::S) {
        translation += camera_transform.back() * distance;
    }
    if keyboard.pressed(KeyCode::D) {
        translation += camera_transform.right() * distance;
    }
    if keyboard.pressed(KeyCode::LShift) {
        translation += camera_transform.up() * distance;
    }
    if keyboard.pressed(KeyCode::LControl) {
        translation += camera_transform.down() * distance;
    }
    if translation != Vec3::ZERO {
        camera_transform.translation += translation;
    }

    if let Some(delta) = mouse.iter().map(|e| e.delta).reduce(|sum, i| sum + i) {
        camera_transform.rotate_local_x(delta.y * -0.001);
        camera_transform.rotate_local_y(delta.x * -0.001);
    }
}
