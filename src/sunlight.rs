use bevy::prelude::*;

pub struct SunlightPlugin;

impl Plugin for SunlightPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 4096 })
            .add_startup_system(spawn_sunlight)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_sunlight.after(bevy::transform::TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Component, Debug, Reflect)]
pub(crate) struct SunlightMarker;

#[derive(Component, Debug, Reflect)]
pub struct SunlightCamera;

#[derive(Component, Clone, Debug, Reflect)]
pub struct Sunlight {
    pub(crate) illuminance: f32,
    pub(crate) color: Color,
}

pub fn spawn_sunlight(mut commands: Commands) {
    commands.spawn((directional_light(1024.0), SunlightMarker));
}

fn directional_light(size: f32) -> DirectionalLightBundle {
    DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            shadow_projection: OrthographicProjection {
                left: -size,
                right: size,
                bottom: -size,
                top: size,
                near: -size,
                far: size,
                ..Default::default()
            },
            ..default()
        },
        ..default()
    }
}

pub fn update_sunlight(
    camera: Query<&GlobalTransform, With<SunlightCamera>>,
    suns: Query<(&GlobalTransform, &Sunlight)>,
    mut lights: Query<(&mut Transform, &mut DirectionalLight)>,
) {
    let cam_global = camera.single();
    let mut distance_sq = f32::MAX;
    let mut nearest_sun = None;
    for (sun_global, sun) in &suns {
        let sun_dir = sun_global.affine().translation - cam_global.affine().translation;
        let new_dist = sun_dir.length_squared();
        if new_dist < distance_sq {
            distance_sq = new_dist;
            nearest_sun = Some((sun.to_owned(), Vec3::from(sun_dir)))
        }
    }
    if let Some((sun, sun_dir)) = nearest_sun {
        for (mut light_local, mut directional_light) in lights.iter_mut() {
            let cam_translation = Vec3::from(cam_global.affine().translation);
            let sun_dir = sun_dir.normalize();
            light_local.translation = cam_translation + sun_dir;
            light_local.look_at(cam_translation, Vec3::Y);
            directional_light.illuminance = sun.illuminance / 2.0;
            directional_light.color = sun.color;
        }
    }
}
