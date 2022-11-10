//! Problem: objects far from the origin suffer from reduced precision.
//! Solution: store object position relative to the current grid cell. Each grid cell should be about 10km on each edge to give 0.5mm precision at the extents.
//! Store grid cells in an octree.
//! When an object exceeds its boundary,

use bevy::{math::DVec3, prelude::*, transform::TransformSystem};
use bevy_inspector_egui::Inspectable;
use bevy_polyline::prelude::{Polyline, PolylineBundle, PolylineMaterial};
use std::marker::PhantomData;

pub mod precision;
use precision::*;

#[derive(Default)]
pub struct FloatingOriginPlugin<P: Precision>(PhantomData<P>);
impl<P: Precision> Plugin for FloatingOriginPlugin<P> {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_polyline::PolylinePlugin)
            .init_resource::<FloatingOriginSettings>()
            .register_type::<Transform>()
            .register_type::<GlobalTransform>()
            .register_type::<GridPosition<P>>()
            .add_startup_system(spawn_debug_bounds)
            // .add_system(update_debug_bounds)
            // add transform systems to startup so the first update is "correct"
            .add_startup_system_to_stage(StartupStage::PostStartup, grid_recentering::<P>)
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                transform_propagate_system::<P>
                    .label(TransformSystem::TransformPropagate)
                    .after(grid_recentering::<P>),
            )
            .add_system_to_stage(CoreStage::PostUpdate, grid_recentering::<P>)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                transform_propagate_system::<P>
                    .label(TransformSystem::TransformPropagate)
                    .after(grid_recentering::<P>),
            );
    }
}

#[derive(Reflect, Inspectable)]
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
    pub fn pos_double<P: Precision>(&self, pos: &GridPosition<P>, transform: &Transform) -> DVec3 {
        DVec3 {
            x: pos.x.as_f64() * self.grid_cell_edge_length as f64 + transform.translation.x as f64,
            y: pos.y.as_f64() * self.grid_cell_edge_length as f64 + transform.translation.y as f64,
            z: pos.z.as_f64() * self.grid_cell_edge_length as f64 + transform.translation.z as f64,
        }
    }
    pub fn pos_single<P: Precision>(&self, pos: &GridPosition<P>, transform: &Transform) -> Vec3 {
        Vec3 {
            x: pos.x.as_f64() as f32 * self.grid_cell_edge_length + transform.translation.x,
            y: pos.y.as_f64() as f32 * self.grid_cell_edge_length + transform.translation.y,
            z: pos.z.as_f64() as f32 * self.grid_cell_edge_length + transform.translation.z,
        }
    }

    pub fn precise_translation<P: Precision>(&self, input: Vec3) -> (GridPosition<P>, Vec3) {
        let l = self.grid_cell_edge_length as f64;
        let DVec3 { x, y, z } = input.as_dvec3();

        if input.abs().max_element() < self.distance_limit {
            return (GridPosition::default(), input);
        }

        let x_r = (x / l).round();
        let y_r = (y / l).round();
        let z_r = (z / l).round();
        let t_x = x - x_r * l;
        let t_y = y - y_r * l;
        let t_z = z - z_r * l;

        (
            GridPosition {
                x: P::from_f64(x_r),
                y: P::from_f64(y_r),
                z: P::from_f64(z_r),
            },
            Vec3::new(t_x as f32, t_y as f32, t_z as f32),
        )
    }
}

impl Default for FloatingOriginSettings {
    fn default() -> Self {
        Self::new(10_000_f32, 100_f32)
    }
}

pub struct PreciseSpatialBundle<P: Precision> {
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The computed visibility of the entity.
    pub computed: ComputedVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
    /// The grid position of the entity
    pub grid_position: GridPosition<P>,
}

#[derive(Component, Default, Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component, Default, PartialEq)]
pub struct GridPosition<P: Precision> {
    pub x: P,
    pub y: P,
    pub z: P,
}
impl<P: Precision> std::ops::Add for GridPosition<P> {
    type Output = GridPosition<P>;

    fn add(self, rhs: Self) -> Self::Output {
        GridPosition {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
            z: self.z.wrapping_add(rhs.z),
        }
    }
}
impl<P: Precision> std::ops::Sub for GridPosition<P> {
    type Output = GridPosition<P>;

    fn sub(self, rhs: Self) -> Self::Output {
        GridPosition {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
            z: self.z.wrapping_sub(rhs.z),
        }
    }
}
impl<P: Precision> std::ops::Add for &GridPosition<P> {
    type Output = GridPosition<P>;

    fn add(self, rhs: Self) -> Self::Output {
        GridPosition {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
            z: self.z.wrapping_add(rhs.z),
        }
    }
}
impl<P: Precision> std::ops::Sub for &GridPosition<P> {
    type Output = GridPosition<P>;

    fn sub(self, rhs: Self) -> Self::Output {
        GridPosition {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
            z: self.z.wrapping_sub(rhs.z),
        }
    }
}

#[derive(Component)]
pub struct FloatingOriginCamera;

#[derive(Component)]
pub struct DebugBounds;

pub fn spawn_debug_bounds(mut commands: Commands) {
    commands.spawn().insert(DebugBounds);
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

    commands
        .entity(debug_cube.single())
        .insert_bundle(PolylineBundle {
            polyline: polylines.add(Polyline {
                vertices: indices.iter().map(|&i| vertices[i]).collect(),
                ..Default::default()
            }),
            material: polyline_materials.add(PolylineMaterial {
                width: 2.0,
                color: Color::RED,
                perspective: true,
                ..Default::default()
            }),
            ..Default::default()
        });
}

/// If an entity's transform becomes larger than the specified limit, it is relocated to the next
/// grid cell to reduce the size of the transform.
pub fn grid_recentering<P: Precision>(
    settings: Res<FloatingOriginSettings>,
    mut query: Query<(&mut GridPosition<P>, &mut Transform), (Changed<Transform>, Without<Parent>)>,
) {
    query.par_for_each_mut(1024, |(mut grid_pos, mut transform)| {
        let (grid_delta, translation) =
            settings.precise_translation(transform.as_ref().translation);
        *grid_pos = *grid_pos + grid_delta;
        transform.translation = translation;
    });
}

/// Update [`GlobalTransform`] component of entities based on entity hierarchy, [`Transform`], and
/// [`GridPosition`] components.
pub fn transform_propagate_system<P: Precision>(
    origin_settings: Res<FloatingOriginSettings>,
    mut camera: Query<(&GridPosition<P>, Changed<GridPosition<P>>), With<FloatingOriginCamera>>,
    mut root_query: Query<
        (
            Option<(&Children, Changed<Children>)>,
            &Transform,
            Changed<Transform>,
            &GridPosition<P>,
            Changed<GridPosition<P>>,
            &mut GlobalTransform,
            Entity,
        ),
        Without<Parent>,
    >,
    mut transform_query: Query<(
        &Transform,
        Changed<Transform>,
        &mut GlobalTransform,
        &Parent,
    )>,
    children_query: Query<(&Children, Changed<Children>), (With<Parent>, With<GlobalTransform>)>,
) {
    let (cam_grid_pos, cam_grid_pos_changed) = camera.single_mut();

    for (
        children,
        transform,
        transform_changed,
        entity_grid_pos,
        grid_pos_changed,
        mut global_transform,
        entity,
    ) in root_query.iter_mut()
    {
        let mut changed = transform_changed || cam_grid_pos_changed;
        if transform_changed || grid_pos_changed || cam_grid_pos_changed {
            let relative_grid = entity_grid_pos - cam_grid_pos;
            let new_transform = transform
                .clone()
                .with_translation(origin_settings.pos_single(&relative_grid, transform));
            *global_transform = GlobalTransform::from(new_transform);
        }

        if let Some((children, changed_children)) = children {
            // If our `Children` has changed, we need to recalculate everything below us
            changed |= changed_children;
            for child in children {
                let _ = propagate_recursive(
                    &global_transform,
                    &mut transform_query,
                    &children_query,
                    *child,
                    entity,
                    changed,
                );
            }
        }
    }
}

fn propagate_recursive(
    parent: &GlobalTransform,
    transform_query: &mut Query<(
        &Transform,
        Changed<Transform>,
        &mut GlobalTransform,
        &Parent,
    )>,
    children_query: &Query<(&Children, Changed<Children>), (With<Parent>, With<GlobalTransform>)>,
    entity: Entity,
    expected_parent: Entity,
    mut changed: bool,
    // We use a result here to use the `?` operator. Ideally we'd use a try block instead
) -> Result<(), ()> {
    let global_matrix = {
        let (transform, transform_changed, mut global_transform, child_parent) =
            transform_query.get_mut(entity).map_err(drop)?;
        // Note that for parallelising, this check cannot occur here, since there is an `&mut GlobalTransform` (in global_transform)
        assert_eq!(
                child_parent.get(), expected_parent,
        "Malformed hierarchy. This probably means that your hierarchy has been improperly maintained, or contains a cycle"
        );
        changed |= transform_changed;
        if changed {
            *global_transform = parent.mul_transform(*transform);
        }
        *global_transform
    };

    let (children, changed_children) = children_query.get(entity).map_err(drop)?;
    // If our `Children` has changed, we need to recalculate everything below us
    changed |= changed_children;
    for child in children {
        let _ = propagate_recursive(
            &global_matrix,
            transform_query,
            children_query,
            *child,
            entity,
            changed,
        );
    }
    Ok(())
}
