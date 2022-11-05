//! Problem: objects far from the origin suffer from reduced precision.
//! Solution: Store the position of objects

use bevy::{prelude::*, math::{DVec3}};


pub struct FloatingOriginPlugin;
impl Plugin for FloatingOriginPlugin {
    fn build(&self, app: &mut App) {
 
    }
}
/// A double precision transformation isometry.
#[derive(Component)]
pub struct UniverseTransform {
    /// Double precision translation
    translation: DVec3,
    /// Single precision rotation
    rotation: Mat3,
    /// Single precision scale
    scale: Vec3,
}


pub fn precise_transform_propagation(query: Query<(Entity, &mut Transform, &UniverseTransform)>){

}

pub fn camera_recenter(query: Query<&mut Transform, With<UniverseCamera>>) {

}