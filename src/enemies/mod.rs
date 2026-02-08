use crate::{
    character_controls::Character,
    light::{CheckInLight, InLight},
    room::Movable,
};
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct TrackPlayer {
    radius: f32,
    speed: f32,
}

#[derive(Component)]
pub struct Teleport {
    distance: f32,
    chance: f32,
}

pub enum FleeAction {
    Walk(f32),
    Teleport(f32),
}

#[derive(Component)]
pub struct FleeLight {
    action: FleeAction,
}

fn alien(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Alien"),
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

fn teleporting_alien(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Teleporting Alien"),
        CheckInLight(45.0),
        Enemy,
        Movable,
        Velocity::default(),
        TrackPlayer {
            radius: 500.0,
            speed: 20.0,
        },
        Teleport {
            distance: 500.0,
            chance: 0.001,
        },
        FleeLight {
            action: FleeAction::Teleport(50.0),
        },
        Sprite {
            image: asset_server.load("teleporting_alien.png"),
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
    mut q_enemies: Query<
        (&TrackPlayer, &mut Transform, &mut Velocity),
        (
            Without<Character>,
            Or<(Without<InLight>, Without<FleeLight>)>,
        ),
    >,
    mut q_player: Query<&Transform, With<Character>>,
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
        } else {
            enemy_velocity.linear_velocity = enemy_velocity.linear_velocity.lerp(Vec2::ZERO, 0.5);
        }
    }
}

fn teleport(
    mut q_enemies: Query<
        (&Teleport, &mut Transform),
        (
            Without<Character>,
            Or<(Without<InLight>, Without<FleeLight>)>,
        ),
    >,
    mut q_player: Query<&Transform, With<Character>>,
) {
    let player_transform = q_player.single_mut().expect("No Player Object");
    let player_translation = player_transform.translation.truncate();
    let mut rng = rand::rng();
    for (teleport, mut enemy_transform) in q_enemies.iter_mut() {
        let chance: f32 = rng.random();
        if chance < teleport.chance {
            let enemy_translation = enemy_transform.translation.truncate();
            let dir = (player_translation - enemy_translation).normalize_or_zero();
            let random_angle = rng.random_range(-FRAC_PI_2..=FRAC_PI_2);
            let wiggled_dir = Vec2::from_angle(random_angle).rotate(dir);
            let distance_multiplier = rng.random_range(0.5..1.0);
            enemy_transform.translation +=
                (wiggled_dir * (teleport.distance * distance_multiplier)).extend(0.0);
        }
    }
}

fn flee_light(
    mut q_enemies: Query<
        (&FleeLight, &mut Transform, &mut Velocity),
        (With<InLight>, Without<Character>),
    >,
    mut q_player: Query<&Transform, With<Character>>,
) {
    let player_transform = q_player.single_mut().expect("No Player Object");
    let player_translation = player_transform.translation.truncate();
    for (flee_light, mut enemy_transform, mut enemy_velocity) in q_enemies.iter_mut() {
        let enemy_translation = enemy_transform.translation.truncate();
        let dir = (enemy_translation - player_translation).normalize_or_zero();
        match flee_light.action {
            FleeAction::Walk(speed) => {
                enemy_velocity.linear_velocity =
                    enemy_velocity.linear_velocity.lerp(dir * speed, 0.5);
            }
            FleeAction::Teleport(distance) => {
                let mut rng = rand::rng();
                let random_angle = rng.random_range(-FRAC_PI_2..=FRAC_PI_2);
                let wiggled_dir = Vec2::from_angle(random_angle).rotate(dir);
                enemy_transform.translation =
                    enemy_transform.translation + (wiggled_dir * distance).extend(0.0);
                println!("{}", enemy_transform.translation);
            }
        }
    }
}

fn apply_velocity(time: Res<Time>, mut q_enemies: Query<(&mut Transform, &Velocity), With<Enemy>>) {
    let dt = time.delta_secs();
    for (mut trans, vel) in q_enemies.iter_mut() {
        trans.translation.x += vel.linear_velocity.x * dt;
        trans.translation.y += vel.linear_velocity.y * dt;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        alien(&asset_server),
        Transform::from_translation(Vec3::new(-100.0, 150.0, 3.0)),
    ));

    commands.spawn((
        teleporting_alien(&asset_server),
        Transform::from_translation(Vec3::new(250.0, -50.0, 3.0)),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, (teleport, track_player).chain());
    app.add_systems(PostUpdate, apply_velocity);
}
