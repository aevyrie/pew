use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_polyline::prelude::*;

use crate::{precision::GridPrecision, FloatingOrigin, FloatingOriginSettings, GridCell};

#[derive(Default)]
pub struct FloatingOriginDebugPlugin<P: GridPrecision>(PhantomData<P>);
impl<P: GridPrecision> Plugin for FloatingOriginDebugPlugin<P> {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugBoundsVertices>()
            .add_system(update_debug_bounds::<P>)
            .add_system(build_cube);
    }
}

#[derive(Component, Reflect)]
pub struct DebugBounds;

pub fn update_debug_bounds<P: GridPrecision>(
    mut commands: Commands,
    new_grid_entities: Query<(&GridCell<P>, Option<&FloatingOrigin>), Without<DebugBounds>>,
    debug_bounds: Query<Entity, With<DebugBounds>>,
    vertices: Res<DebugBoundsVertices>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    for e in &debug_bounds {
        commands.entity(e).despawn_recursive();
    }
    for (cell, is_origin) in &new_grid_entities {
        commands.spawn((
            PolylineBundle {
                polyline: polylines.add(Polyline {
                    vertices: vertices.0.into(),
                    ..Default::default()
                }),
                material: polyline_materials.add(PolylineMaterial {
                    width: 3.0,
                    color: if is_origin.is_some() {
                        Color::rgb(0.0, 0.0, 2.0)
                    } else {
                        Color::rgb(2.0, 0.0, 0.0)
                    },
                    perspective: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            cell.to_owned(),
            DebugBounds,
        ));
    }
}

#[derive(Resource, Default)]
pub struct DebugBoundsVertices([Vec3; 27]);

pub fn build_cube(
    settings: Res<FloatingOriginSettings>,
    mut debug_vertices: ResMut<DebugBoundsVertices>,
) {
    if !settings.is_changed() {
        return;
    }

    let s = settings.grid_edge_length / 2.0;

    /*
        (2)-----(3)               Y
         | \     | \              |
         |  (1)-----(0) MAX       o---X
         |   |   |   |             \
    MIN (6)--|--(7)  |              Z
           \ |     \ |
            (5)-----(4)
     */

    let indices = [
        0, 1, 1, 2, 2, 3, 3, 0, // Top ring
        4, 5, 5, 6, 6, 7, 7, 4, // Bottom ring
        0, 4, 8, 1, 5, 8, 2, 6, 8, 3, 7, // Verticals (8's are NaNs)
    ];

    let vertices = [
        Vec3::new(s, s, s),
        Vec3::new(-s, s, s),
        Vec3::new(-s, s, -s),
        Vec3::new(s, s, -s),
        Vec3::new(s, -s, s),
        Vec3::new(-s, -s, s),
        Vec3::new(-s, -s, -s),
        Vec3::new(s, -s, -s),
        Vec3::NAN,
    ];

    debug_vertices.0 = [
        vertices[indices[0]],
        vertices[indices[1]],
        vertices[indices[2]],
        vertices[indices[3]],
        vertices[indices[4]],
        vertices[indices[5]],
        vertices[indices[6]],
        vertices[indices[7]],
        vertices[indices[8]],
        vertices[indices[9]],
        vertices[indices[10]],
        vertices[indices[11]],
        vertices[indices[12]],
        vertices[indices[13]],
        vertices[indices[14]],
        vertices[indices[15]],
        vertices[indices[16]],
        vertices[indices[17]],
        vertices[indices[18]],
        vertices[indices[19]],
        vertices[indices[20]],
        vertices[indices[21]],
        vertices[indices[22]],
        vertices[indices[23]],
        vertices[indices[24]],
        vertices[indices[25]],
        vertices[indices[26]],
    ];
}
