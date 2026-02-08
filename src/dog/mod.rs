use bevy::prelude::*;

use crate::{
    character_controls::{Character, Velocity},
    room::Movable,
};

#[derive(Component)]
pub struct Dog;

#[derive(Component)]
pub struct TrackPlayer {
    outer_radius: f32,
    inner_radius: f32,
    speed: f32,
}

fn dog(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Dog"),
        Dog,
        Movable,
        Velocity::default(),
        TrackPlayer {
            outer_radius: 100.0,
            inner_radius: 50.0,
            speed: 200.0,
        },
        Sprite {
            image: asset_server.load("dog.png"),
            custom_size: Some(Vec2::splat(45.0)),
            ..Default::default()
        },
    )
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

        if difference.length() <= track_player.outer_radius
            && difference.length() >= track_player.inner_radius
        {
            let dir = difference.normalize_or_zero();

            enemy_velocity.linear_velocity = enemy_velocity
                .linear_velocity
                .lerp(dir * track_player.speed, 0.5);

            // If we want to change the dog sprites based on direction.
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

fn apply_velocity(time: Res<Time>, mut q_dog: Query<(&mut Transform, &Velocity), With<Dog>>) {
    let dt = time.delta_secs();
    for (mut trans, vel) in q_dog.iter_mut() {
        trans.translation.x += vel.linear_velocity.x * dt;
        trans.translation.y += vel.linear_velocity.y * dt;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        dog(&asset_server),
        Transform::from_translation(Vec3::new(250.0, 800.0, 3.0)),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, track_player);
    app.add_systems(PostUpdate, apply_velocity);
}
