use bevy::prelude::*;

/// Component that marks an entity for Y-sorting
/// Objects with lower Y positions render behind objects with higher Y positions
#[derive(Component)]
pub struct YSort {
    /// Base Z layer (0.0 for ground level, negative for backgrounds)
    pub base_z: f32,
    /// How much to scale Y position (typically 0.001 - 0.01)
    pub y_scale: f32,
}

impl YSort {
    /// Create a YSort component with default settings
    pub fn default_layer() -> Self {
        Self {
            base_z: 0.0,
            y_scale: 0.001,
        }
    }
    
    /// Create a YSort component with custom base Z
    pub fn with_base_z(base_z: f32) -> Self {
        Self {
            base_z,
            y_scale: 0.001,
        }
    }
}

/// System that updates Z coordinates based on Y position
/// This creates automatic depth sorting
pub fn y_sort_system(
    mut query: Query<(&mut Transform, &YSort)>,
) {
    for (mut transform, y_sort) in query.iter_mut() {
        // Calculate Z based on Y position
        // Lower Y = further back = lower Z = renders behind
        // Higher Y = closer = higher Z = renders in front
        transform.translation.z = y_sort.base_z - (transform.translation.y * y_sort.y_scale);
    }
}

pub(super) fn register(app: &mut App) {
    // Run in PostUpdate so it happens after movement but before rendering
    app.add_systems(PostUpdate, y_sort_system);
}