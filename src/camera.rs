use bevy::{input::mouse::MouseMotion, prelude::*, render::primitives::Aabb};

pub struct CameraControllerPlugin;
impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_controller);
    }
}

#[derive(Component)]
pub struct CameraController {
    pub top_speed: f32,
    pub jerk: f32,
}
impl CameraController {
    pub(crate) fn new(top_speed: f32, jerk: f32) -> CameraController {
        CameraController { top_speed, jerk }
    }
}

#[derive(Component)]
pub struct IgnoreCamDist;

pub fn camera_controller(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut mouse: EventReader<MouseMotion>,
    mut camera: Query<(&mut Transform, &GlobalTransform, &mut CameraController)>,
    mut current_speed: Local<Vec3>,
    objects: Query<(&GlobalTransform, &Aabb), Without<IgnoreCamDist>>,
    mut camera_target: Local<Transform>,
) {
    let (mut camera_transform, cam_global_transform, controller) = camera.single_mut();

    let mut nearest_object = f32::MAX;
    for (transform, aabb) in &objects {
        let distance = (transform.translation() + Vec3::from(aabb.center)
            - cam_global_transform.translation())
        .length()
            - aabb.half_extents.max_element();
        if distance < 0.0 {
            continue;
        }
        nearest_object = nearest_object.min(distance);
    }

    let top_speed = nearest_object.clamp(50.0, controller.top_speed);

    let mut target = Vec3::ZERO;

    if keyboard.pressed(KeyCode::W) {
        target += camera_transform.forward();
    }
    if keyboard.pressed(KeyCode::S) {
        target += camera_transform.back();
    }

    if keyboard.pressed(KeyCode::A) {
        target += camera_transform.left();
    }
    if keyboard.pressed(KeyCode::D) {
        target += camera_transform.right();
    }

    if keyboard.pressed(KeyCode::Space) {
        target += camera_transform.up();
    }
    if keyboard.pressed(KeyCode::LControl) {
        target += camera_transform.down();
    }

    if keyboard.pressed(KeyCode::Q) {
        camera_target.rotate_local_z(0.02);
    }
    if keyboard.pressed(KeyCode::E) {
        camera_target.rotate_local_z(-0.02);
    }

    target *= top_speed;

    let actual = *current_speed;
    let error = target - actual;
    let p = 200.0 * error;

    let acceleration = time.delta_seconds() * p;
    *current_speed += time.delta_seconds() * acceleration;
    camera_transform.translation += time.delta_seconds() * *current_speed;

    if let Some(delta) = mouse.iter().map(|e| e.delta).reduce(|sum, i| sum + i) {
        camera_target.rotate_local_x(delta.y * -0.003);
        camera_target.rotate_local_y(delta.x * -0.003);
    }

    camera_transform.rotation = camera_transform
        .rotation
        .slerp(camera_target.rotation, 0.2)
        .normalize();
}
