use crate::character_controls::{Character, Velocity};
use bevy::prelude::*;
pub mod room_objects;

/// Component that marks an entity as having collision
#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
    /// Offset from the entity's transform position (useful for tree bases, etc.)
    pub offset: Vec2,
}

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: Vec2::new(width, height),
            offset: Vec2::ZERO,
        }
    }

    pub fn square(size: f32) -> Self {
        Self::new(size, size)
    }

    /// Create a collider with an offset (useful for trees - small collision at base)
    pub fn with_offset(width: f32, height: f32, offset: Vec2) -> Self {
        Self {
            size: Vec2::new(width, height),
            offset,
        }
    }

    /// Get the actual collision position (transform position + offset)
    pub fn collision_position(&self, transform_pos: Vec3) -> Vec3 {
        transform_pos + self.offset.extend(0.0)
    }
}

/// Component for static world objects (foliage, spaceships, etc.)
#[derive(Component)]
pub struct WorldObject {
    pub object_type: ObjectType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Foliage,
    CrashedShip,
    Rock,
    Crystal,
    Debris,
    Body
}

/// Bundle for spawning world objects with collision
#[derive(Bundle)]
pub struct WorldObjectBundle {
    pub world_object: WorldObject,
    pub collider: Collider,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl WorldObjectBundle {
    /// Create a new world object at the specified position
    pub fn new(
        object_type: ObjectType,
        position: Vec3,
        size: Vec2,
        texture: Handle<Image>,
    ) -> Self {
        Self {
            world_object: WorldObject { object_type },
            collider: Collider::new(size.x, size.y),
            sprite: Sprite {
                image: texture,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(position),
        }
    }

    /// Create a world object with offset collision (useful for trees)
    /// The collision box will be offset from the sprite's center
    pub fn with_collision_offset(
        object_type: ObjectType,
        position: Vec3,
        sprite_size: Vec2,
        collision_size: Vec2,
        collision_offset: Vec2,
        texture: Handle<Image>,
    ) -> Self {
        Self {
            world_object: WorldObject { object_type },
            collider: Collider::with_offset(collision_size.x, collision_size.y, collision_offset),
            sprite: Sprite {
                image: texture,
                custom_size: Some(sprite_size),
                ..default()
            },
            transform: Transform::from_translation(position),
        }
    }
}

/// Structure to define object placements from data
pub struct ObjectPlacement {
    pub object_type: ObjectType,
    pub position: Vec2,
    pub size: Vec2,
    pub sprite_path: String,
    /// Optional: Different collision size (if None, uses sprite size)
    pub collision_size: Option<Vec2>,
    /// Optional: Offset for collision box (useful for trees)
    pub collision_offset: Option<Vec2>,
}

/// Check if two AABBs (Axis-Aligned Bounding Boxes) overlap
pub fn check_collision(pos1: Vec3, size1: Vec2, pos2: Vec3, size2: Vec2) -> bool {
    let half_size1 = size1 / 2.0;
    let half_size2 = size2 / 2.0;

    let min1 = Vec2::new(pos1.x - half_size1.x, pos1.y - half_size1.y);
    let max1 = Vec2::new(pos1.x + half_size1.x, pos1.y + half_size1.y);

    let min2 = Vec2::new(pos2.x - half_size2.x, pos2.y - half_size2.y);
    let max2 = Vec2::new(pos2.x + half_size2.x, pos2.y + half_size2.y);

    max1.x > min2.x && min1.x < max2.x && max1.y > min2.y && min1.y < max2.y
}

/// System that handles collision detection between player and world objects
/// This should run AFTER velocity is applied but BEFORE camera update
pub fn collision_detection_system(
    mut player_query: Query<(&mut Transform, &Collider, &mut Velocity), With<Character>>,
    object_query: Query<(&Transform, &Collider), (With<WorldObject>, Without<Character>)>,
) {
    if let Ok((mut player_transform, player_collider, mut player_velocity)) =
        player_query.single_mut()
    {
        let player_collision_pos = player_collider.collision_position(player_transform.translation);

        // Check collision with all world objects
        for (object_transform, object_collider) in object_query.iter() {
            let object_collision_pos =
                object_collider.collision_position(object_transform.translation);

            if check_collision(
                player_collision_pos,
                player_collider.size,
                object_collision_pos,
                object_collider.size,
            ) {
                // Calculate push-back direction
                let diff = player_collision_pos.truncate() - object_collision_pos.truncate();
                let push_dir = diff.normalize_or_zero();

                // Calculate penetration depth
                let half_sizes = (player_collider.size + object_collider.size) / 2.0;
                let abs_diff = diff.abs();
                let penetration = half_sizes - abs_diff;

                // Push player out in the direction of least resistance
                if penetration.x < penetration.y {
                    player_transform.translation.x += push_dir.x * penetration.x;
                } else {
                    player_transform.translation.y += push_dir.y * penetration.y;
                }

                // Zero out velocity in collision direction to prevent sliding
                if penetration.x < penetration.y {
                    player_velocity.linear_velocity.x *= 0.1;
                } else {
                    player_velocity.linear_velocity.y *= 0.1;
                }
            }
        }
    }
}

/// Spawn world objects from a list of placements
pub fn spawn_objects_from_data(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    placements: &[ObjectPlacement],
) {
    for placement in placements {
        let texture = asset_server.load(&placement.sprite_path);
        let position = Vec3::new(placement.position.x, placement.position.y, 1.0);

        // If collision offset is specified, use the offset version
        if let (Some(collision_size), Some(collision_offset)) =
            (placement.collision_size, placement.collision_offset)
        {
            commands.spawn(WorldObjectBundle::with_collision_offset(
                placement.object_type,
                position,
                placement.size,
                collision_size,
                collision_offset,
                texture,
            ));
        } else if let Some(collision_size) = placement.collision_size {
            // Different collision size but no offset
            commands.spawn(WorldObjectBundle {
                world_object: WorldObject {
                    object_type: placement.object_type,
                },
                collider: Collider::new(collision_size.x, collision_size.y),
                sprite: Sprite {
                    image: texture,
                    custom_size: Some(placement.size),
                    ..default()
                },
                transform: Transform::from_translation(position),
            });
        } else {
            // Standard placement (collision matches sprite)
            commands.spawn(WorldObjectBundle::new(
                placement.object_type,
                position,
                placement.size,
                texture,
            ));
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(
        PostUpdate,
        collision_detection_system.after(crate::character_controls::apply_velocity),
    );
}
