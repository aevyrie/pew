//! Problem: objects far from the origin suffer from reduced precision.
//! Solution: store object position relative to the current grid cell. Each grid cell should be about 10km on each edge to give 0.5mm precision at the extents.
//! Store grid cells in an octree.
//! When an object exceeds its boundary,

use bevy::{math::DVec3, prelude::*, transform::TransformSystem};
use bevy_polyline::prelude::{Polyline, PolylineBundle, PolylineMaterial};
use std::marker::PhantomData;

pub mod precision;
use precision::*;

#[derive(Default)]
pub struct FloatingOriginPlugin<I: Index>(PhantomData<I>);
impl<I: Index> Plugin for FloatingOriginPlugin<I> {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_polyline::PolylinePlugin)
            .init_resource::<FloatingOriginSettings>()
            .register_type::<Transform>()
            .register_type::<GlobalTransform>()
            .register_type::<GridCell<I>>()
            .add_startup_system(spawn_debug_bounds)
            .add_system(update_debug_bounds)
            // add transform systems to startup so the first update is "correct"
            .add_startup_system_to_stage(StartupStage::PostStartup, update_grid_origin::<I>)
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                update_position_from_grid::<I>
                    .after(update_grid_origin::<I>)
                    .before(bevy::transform::transform_propagate_system)
                    .label(TransformSystem::TransformPropagate),
            )
            .add_system_to_stage(CoreStage::PostUpdate, update_grid_origin::<I>)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_position_from_grid::<I>
                    .after(update_grid_origin::<I>)
                    .before(bevy::transform::transform_propagate_system)
                    .label(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Reflect, Resource)]
pub struct FloatingOriginSettings {
    grid_cell_edge_length: f32,
    distance_limit: f32,
}

impl FloatingOriginSettings {
    /// # `switching_threshold`:
    ///
    /// How far past the extents of a cell an entity must travel before a grid recentering occurs.
    /// This prevents entities from rapidly switching between cells when moving along a boundary.
    pub fn new(grid_cell_edge_length: f32, switching_threshold: f32) -> Self {
        Self {
            grid_cell_edge_length,
            distance_limit: grid_cell_edge_length / 2.0 + switching_threshold,
        }
    }

    /// Converts the
    pub fn pos_double<I: Index>(&self, pos: &GridCell<I>, transform: &Transform) -> DVec3 {
        DVec3 {
            x: pos.x.as_f64() * self.grid_cell_edge_length as f64 + transform.translation.x as f64,
            y: pos.y.as_f64() * self.grid_cell_edge_length as f64 + transform.translation.y as f64,
            z: pos.z.as_f64() * self.grid_cell_edge_length as f64 + transform.translation.z as f64,
        }
    }
    pub fn pos_single<I: Index>(&self, pos: &GridCell<I>, transform: &Transform) -> Vec3 {
        Vec3 {
            x: pos.x.as_f64() as f32 * self.grid_cell_edge_length + transform.translation.x,
            y: pos.y.as_f64() as f32 * self.grid_cell_edge_length + transform.translation.y,
            z: pos.z.as_f64() as f32 * self.grid_cell_edge_length + transform.translation.z,
        }
    }

    pub fn precise_translation<I: Index>(&self, input: DVec3) -> (GridCell<I>, Vec3) {
        let l = self.grid_cell_edge_length as f64;
        let DVec3 { x, y, z } = input;

        if input.abs().max_element() < self.distance_limit as f64 {
            return (GridCell::default(), input.as_vec3());
        }

        let x_r = (x / l).round();
        let y_r = (y / l).round();
        let z_r = (z / l).round();
        let t_x = x - x_r * l;
        let t_y = y - y_r * l;
        let t_z = z - z_r * l;

        (
            GridCell {
                x: I::from_f64(x_r),
                y: I::from_f64(y_r),
                z: I::from_f64(z_r),
            },
            Vec3::new(t_x as f32, t_y as f32, t_z as f32),
        )
    }
}

impl Default for FloatingOriginSettings {
    fn default() -> Self {
        Self::new(10_000f32, 100f32)
    }
}

pub struct PreciseSpatialBundle<I: Index> {
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The computed visibility of the entity.
    pub computed: ComputedVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
    /// The grid position of the entity
    pub grid_position: GridCell<I>,
}

/// Defines the grid cell this entity's `Transform` is relative to.
///
/// `GridPosition` is generic over a few integer types to allow you to select the grid size you
/// need. Assuming you are using a grid cell edge length of 10,000 meters, these correspond with:
///
/// - i8: 2,560 km = 74% of the diameter of the Moon
/// - i16 655,350 km = 85% of the diameter of the Moon's orbit around Earth
/// - i32: 0.0045 light years = ~4 times the width of the solar system
/// - i64: 19.5 million light years = ~100 times the width of the milky way galaxy
/// - i128: 3.6e+26 light years = ~3.9e+15 times the width of the observable universe
///
/// # Note
///
/// Be sure you are using the same grid index precision everywhere. It might be a good idea to
/// define a type alias!
///
/// ```
/// type GalacticGrid = GridCell<i64>;
/// ```
///
#[derive(Component, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Reflect)]
#[reflect(Component, Default, PartialEq)]
pub struct GridCell<I: Index> {
    pub x: I,
    pub y: I,
    pub z: I,
}

impl<I: Index> GridCell<I> {
    pub const ZERO: Self = GridCell {
        x: I::ZERO,
        y: I::ZERO,
        z: I::ZERO,
    };
    pub const ONE: Self = GridCell {
        x: I::ONE,
        y: I::ONE,
        z: I::ONE,
    };
}
impl<I: Index> std::ops::Add for GridCell<I> {
    type Output = GridCell<I>;

    fn add(self, rhs: Self) -> Self::Output {
        GridCell {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
            z: self.z.wrapping_add(rhs.z),
        }
    }
}
impl<I: Index> std::ops::Sub for GridCell<I> {
    type Output = GridCell<I>;

    fn sub(self, rhs: Self) -> Self::Output {
        GridCell {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
            z: self.z.wrapping_sub(rhs.z),
        }
    }
}
impl<I: Index> std::ops::Add for &GridCell<I> {
    type Output = GridCell<I>;

    fn add(self, rhs: Self) -> Self::Output {
        (*self).add(*rhs)
    }
}
impl<I: Index> std::ops::Sub for &GridCell<I> {
    type Output = GridCell<I>;

    fn sub(self, rhs: Self) -> Self::Output {
        (*self).sub(*rhs)
    }
}

#[derive(Component)]
pub struct FloatingOrigin;

#[derive(Component)]
pub struct DebugBounds;

pub fn spawn_debug_bounds(mut commands: Commands) {
    commands.spawn(DebugBounds);
}

pub fn update_debug_bounds(
    settings: Res<FloatingOriginSettings>,
    debug_cube: Query<Entity, With<DebugBounds>>,
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    if !settings.is_changed() {
        return;
    }

    let s = settings.grid_cell_edge_length / 2.0;

    /*
        (2)-----(3)               Y
         | \     | \              |
         |  (1)-----(0) MAX       o---X
         |   |   |   |             \
    MIN (6)--|--(7)  |              Z
           \ |     \ |
            (5)-----(4)
     */

    let indices = vec![
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

    commands.entity(debug_cube.single()).insert(PolylineBundle {
        polyline: polylines.add(Polyline {
            vertices: indices.iter().map(|&i| vertices[i]).collect(),
            ..Default::default()
        }),
        material: polyline_materials.add(PolylineMaterial {
            width: 3.0,
            color: Color::rgb(1.8, 0., 0.),
            perspective: false,
            ..Default::default()
        }),
        ..Default::default()
    });
}

/// If an entity's transform becomes larger than the specified limit, it is relocated to the next
/// grid cell to reduce the size of the transform.
pub fn update_grid_origin<I: Index>(
    settings: Res<FloatingOriginSettings>,
    mut query: Query<(&mut GridCell<I>, &mut Transform), (Changed<Transform>, Without<Parent>)>,
) {
    query.par_for_each_mut(10_000, |(mut grid_pos, mut transform)| {
        if transform.as_ref().translation.abs().max_element() > settings.distance_limit {
            let (grid_delta, translation) =
                settings.precise_translation(transform.as_ref().translation.as_dvec3());
            *grid_pos = *grid_pos + grid_delta;
            transform.translation = translation;
        }
    });
}

pub fn update_position_from_grid<I: Index>(
    origin_settings: Res<FloatingOriginSettings>,
    mut origin: Query<(&GridCell<I>, Changed<GridCell<I>>), With<FloatingOrigin>>,
    mut entities: ParamSet<(
        Query<
            (&Transform, &GridCell<I>, &mut GlobalTransform),
            Or<(Changed<Transform>, Changed<GridCell<I>>)>,
        >,
        Query<(&Transform, &GridCell<I>, &mut GlobalTransform)>,
    )>,
) {
    let (origin_pos, origin_grid_pos_changed) = origin.single_mut();

    if origin_grid_pos_changed {
        let mut all_entities = entities.p1();
        all_entities.par_for_each_mut(
            10_000,
            |(transform, entity_grid_pos, mut global_transform)| {
                let grid_pos_delta = entity_grid_pos - origin_pos;
                if grid_pos_delta != GridCell::ZERO {
                    let new_transform = transform
                        .clone()
                        .with_translation(origin_settings.pos_single(&grid_pos_delta, transform));
                    *global_transform = GlobalTransform::from(new_transform);
                }
            },
        );
    } else {
        let mut changed_entities = entities.p0();
        changed_entities.par_for_each_mut(
            10_000,
            |(transform, entity_grid_pos, mut global_transform)| {
                let grid_pos_delta = entity_grid_pos - origin_pos;
                if grid_pos_delta != GridCell::ZERO {
                    let new_transform = transform
                        .clone()
                        .with_translation(origin_settings.pos_single(&grid_pos_delta, transform));
                    *global_transform = GlobalTransform::from(new_transform);
                }
            },
        );
    }
}
