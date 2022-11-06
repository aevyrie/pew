//! Problem: objects far from the origin suffer from reduced precision.
//! Solution: store object position relative to the current grid cell. Each grid cell should be about 10km on each edge to give 0.5mm precision at the extents.
//! Store grid cells in an octree.
//! When an object exceeds its boundary,

use bevy::{math::DVec3, prelude::*, transform::TransformSystem};
use std::marker::PhantomData;

pub mod precision;
use precision::*;

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

#[derive(Default)]
pub struct FloatingOriginPlugin<P: Precision>(PhantomData<P>);
impl<P: Precision> Plugin for FloatingOriginPlugin<P> {
    fn build(&self, app: &mut App) {
        app.init_resource::<FloatingOriginSettings>()
            .register_type::<Transform>()
            .register_type::<GlobalTransform>()
            .register_type::<GridPosition<P>>()
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

pub fn grid_recentering<P: Precision>(
    settings: Res<FloatingOriginSettings>,
    mut query: Query<(&mut GridPosition<P>, &mut Transform), (Changed<Transform>, Without<Parent>)>,
) {
    query.par_for_each_mut(1, |(mut grid_pos, mut transform)| {
        let limit = settings.distance_limit;
        let edge_length = settings.grid_cell_edge_length;

        while transform.as_ref().translation.x > limit {
            grid_pos.x = grid_pos.x.wrapping_add(P::one());
            transform.translation.x -= edge_length;
        }
        while transform.as_ref().translation.y > limit {
            grid_pos.y = grid_pos.y.wrapping_add(P::one());
            transform.translation.y -= edge_length;
        }
        while transform.as_ref().translation.z > limit {
            grid_pos.z = grid_pos.z.wrapping_add(P::one());
            transform.translation.z -= edge_length;
        }
        while transform.as_ref().translation.x < -limit {
            grid_pos.x = grid_pos.x.wrapping_sub(P::one());
            transform.translation.x += edge_length;
        }
        while transform.as_ref().translation.y < -limit {
            grid_pos.y = grid_pos.y.wrapping_sub(P::one());
            transform.translation.y += edge_length;
        }
        while transform.as_ref().translation.z < -limit {
            grid_pos.z = grid_pos.z.wrapping_sub(P::one());
            transform.translation.z += edge_length;
        }
    });
}

/// Update [`GlobalTransform`] component of entities based on entity hierarchy, [`Transform`], and
/// [`GridPosition`] components.
pub fn transform_propagate_system<P: Precision>(
    origin_settings: Res<FloatingOriginSettings>,
    mut camera: Query<&GridPosition<P>, With<FloatingOriginCamera>>,
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
    let cam_grid_pos = camera.single_mut();

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
        let mut changed = transform_changed;
        if transform_changed || grid_pos_changed {
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
