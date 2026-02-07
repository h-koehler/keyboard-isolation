use crate::{
    character_controls::Character,
    light::CheckInLight,
    room::{Movable, ROOM_HEIGHT, ROOM_WIDTH},
    ui::UI_HEIGHT,
};
use bevy::prelude::*;

const ROOM_INSET: f32 = 4.0;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct TrackPlayer {
    radius: f32,
    speed: f32,
}

fn alien(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Enemy"),
        CheckInLight(45.0),
        Enemy,
        Movable,
        Velocity::default(),
        TrackPlayer {
            radius: 300.0,
            speed: 100.0,
        },
        Sprite {
            image: asset_server.load("alien.png"),
            custom_size: Some(Vec2::splat(45.0)),
            ..Default::default()
        },
    )
}

#[derive(Component, Default)]
pub struct Velocity {
    pub linear_velocity: Vec2,
}

fn track_player(
    mut q_enemies: Query<(&TrackPlayer, &mut Transform, &mut Velocity), Without<Character>>,
    mut q_player: Query<&Transform, With<Character>>,
    // profiles: Res<PlayerProfiles>,
) {
    let player_transform = q_player.single_mut().expect("No Player Object");
    let player_translation = player_transform.translation.truncate();
    for (track_player, enemy_transform, mut enemy_velocity) in q_enemies.iter_mut() {
        let enemy_translation = enemy_transform.translation.truncate();
        let difference = player_translation - enemy_translation;
        if difference.length() <= track_player.radius {
            let dir = difference.normalize_or_zero();
            enemy_velocity.linear_velocity = enemy_velocity
                .linear_velocity
                .lerp(dir * track_player.speed, 0.5);

            // If we want to change the enemy sprites based on direction.
            // let dir_abs = dir.abs();
            // let x_greater_than_y = dir_abs.x > dir_abs.y;
            // sprite.image = if x_greater_than_y {
            //     if dir.x > 0.0 {
            //         profiles.right.clone()
            //     } else {
            //         profiles.left.clone()
            //     }
            // } else {
            //     if dir.y > 0.0 {
            //         profiles.up.clone()
            //     } else {
            //         profiles.down.clone()
            //     }
            // };
        } else {
            enemy_velocity.linear_velocity = enemy_velocity.linear_velocity.lerp(Vec2::ZERO, 0.5);
        }
    }
}

fn apply_velocity(
    time: Res<Time>,
    mut q_enemies: Query<(&mut Transform, &Velocity, &Sprite), With<Enemy>>,
) {
    let dt = time.delta_secs();
    for (mut trans, vel, sprite) in q_enemies.iter_mut() {
        trans.translation.x += vel.linear_velocity.x * dt;
        trans.translation.y += vel.linear_velocity.y * dt;

        let half_width = ROOM_WIDTH as f32 / 2.0;
        let half_height = ROOM_HEIGHT as f32 / 2.0;

        let sprite_size = sprite
            .custom_size
            .expect("Expected enemy sprite to have custom size");
        let (half_player_width, half_player_height) = (sprite_size.x * 0.5, sprite_size.y * 0.5);

        let min_x = -half_width + half_player_width + ROOM_INSET;
        let max_x = half_width - half_player_width - ROOM_INSET;
        let min_y = UI_HEIGHT / 2.0 + -half_height + half_player_height + ROOM_INSET;
        let max_y = UI_HEIGHT / 2.0 + half_height - half_player_height - ROOM_INSET;

        trans.translation.x = trans.translation.x.clamp(min_x, max_x);
        trans.translation.y = trans.translation.y.clamp(min_y, max_y);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        alien(&asset_server),
        Transform::from_translation(Vec3::new(-100.0, 150.0, 3.0)),
    ));

    commands.spawn((
        alien(&asset_server),
        Transform::from_translation(Vec3::new(250.0, -50.0, 3.0)),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, track_player);
    app.add_systems(PostUpdate, apply_velocity);
}
