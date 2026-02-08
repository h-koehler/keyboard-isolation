use crate::{
    character_controls::{Character, StatusEffect, StatusEffects},
    light::InLight,
    room::Movable,
};
use bevy::prelude::*;
use bevy_lit::prelude::LightOccluder2d;
use rand::Rng;
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct TrackPlayer {
    max_radius: f32,
    min_radius: f32,
    speed: f32,
}

#[derive(Component)]
pub struct Teleport {
    max_radius: f32,
    min_radius: f32,
    distance: f32,
    chance: f32,
}

pub enum FleeAction {
    Walk {
        speed: f32,
        maybe_direction: Option<Vec2>,
        change_direction_chance: f32,
    },
    Teleport {
        distance: f32,
        chance: f32,
    },
}

#[derive(Component)]
pub struct FleeLight {
    action: FleeAction,
}

#[derive(Component)]
pub struct Attack {
    radius: f32,
    inflicts_status: Option<StatusEffect>,
    cooldown: Timer,
}

#[derive(Component)]
pub struct Stalk {
    radius: f32,
}

fn alien(asset_server: &AssetServer, meshes: &mut Assets<Mesh>) -> impl Bundle {
    (
        Name::new("Alien"),
        Movable,
        Velocity::default(),
        TrackPlayer {
            max_radius: 300.0,
            min_radius: 40.0,
            speed: 100.0,
        },
        Attack {
            radius: 45.0,
            inflicts_status: Some(StatusEffect::Slowed),
            cooldown: Timer::from_seconds(2.0, TimerMode::Once),
        },
        FleeLight {
            action: FleeAction::Walk {
                speed: 300.0,
                maybe_direction: None,
                change_direction_chance: 0.01,
            },
        },
        Mesh2d(meshes.add(Rectangle::new(45.0, 45.0))),
        LightOccluder2d {
            occluder_mask: asset_server.load("alien.png"),
        },
        Sprite {
            image: asset_server.load("alien.png"),
            custom_size: Some(Vec2::splat(45.0)),
            ..Default::default()
        },
    )
}

fn stalker(asset_server: &AssetServer, meshes: &mut Assets<Mesh>) -> impl Bundle {
    (
        Name::new("Stalker"),
        Movable,
        Velocity::default(),
        TrackPlayer {
            max_radius: f32::INFINITY,
            min_radius: 300.0,
            speed: 20.0,
        },
        Stalk { radius: 350.0 },
        FleeLight {
            action: FleeAction::Walk {
                speed: 500.0,
                maybe_direction: None,
                change_direction_chance: 0.05,
            },
        },
        Mesh2d(meshes.add(Rectangle::new(45.0, 45.0))),
        LightOccluder2d {
            occluder_mask: asset_server.load("stalker.png"),
        },
        Sprite {
            image: asset_server.load("stalker.png"),
            custom_size: Some(Vec2::splat(45.0)),
            ..Default::default()
        },
    )
}

fn teleporting_alien(asset_server: &AssetServer, meshes: &mut Assets<Mesh>) -> impl Bundle {
    (
        Name::new("Teleporting Alien"),
        Movable,
        Velocity::default(),
        TrackPlayer {
            max_radius: 500.0,
            min_radius: 40.0,
            speed: 20.0,
        },
        Teleport {
            max_radius: 750.0,
            min_radius: 50.0,
            distance: 500.0,
            chance: 0.001,
        },
        Attack {
            radius: 45.0,
            inflicts_status: Some(StatusEffect::Blind),
            cooldown: Timer::from_seconds(2.0, TimerMode::Once),
        },
        FleeLight {
            action: FleeAction::Teleport {
                distance: 500.0,
                chance: 0.03,
            },
        },
        Mesh2d(meshes.add(Rectangle::new(45.0, 45.0))),
        LightOccluder2d {
            occluder_mask: asset_server.load("teleporting_alien.png"),
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
        let distance = difference.length();
        if track_player.min_radius <= distance && distance <= track_player.max_radius {
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
        let enemy_translation = enemy_transform.translation.truncate();
        let difference = player_translation - enemy_translation;
        let distance = difference.length();
        if teleport.min_radius <= distance && distance <= teleport.max_radius {
            let chance: f32 = rng.random();
            if chance < teleport.chance {
                let dir = (player_translation - enemy_translation).normalize_or_zero();
                let random_angle = rng.random_range(-FRAC_PI_2..=FRAC_PI_2);
                let wiggled_dir = Vec2::from_angle(random_angle).rotate(dir);
                let distance_multiplier = rng.random_range(0.5..1.0);
                enemy_transform.translation +=
                    (wiggled_dir * (teleport.distance * distance_multiplier)).extend(0.0);
            }
        }
    }
}

fn flee_light(
    mut q_enemies: Query<
        (&mut FleeLight, &mut Transform, &mut Velocity),
        (With<InLight>, Without<Character>),
    >,
    mut q_player: Query<&Transform, With<Character>>,
) {
    let player_transform = q_player.single_mut().expect("No Player Object");
    let player_translation = player_transform.translation.truncate();
    for (mut flee_light, mut enemy_transform, mut enemy_velocity) in q_enemies.iter_mut() {
        let enemy_translation = enemy_transform.translation.truncate();
        let dir = (enemy_translation - player_translation).normalize_or_zero();
        let mut rng = rand::rng();
        let random_angle = rng.random_range(-FRAC_PI_2..=FRAC_PI_2);
        let wiggled_dir = Vec2::from_angle(random_angle).rotate(dir);
        let random: f32 = rng.random();
        match &mut flee_light.action {
            FleeAction::Walk {
                speed,
                maybe_direction,
                change_direction_chance,
            } => {
                if random < *change_direction_chance {
                    *maybe_direction = Some(wiggled_dir);
                }
                let dir = if let Some(direction) = maybe_direction {
                    *direction
                } else {
                    *maybe_direction = Some(wiggled_dir);
                    wiggled_dir
                };
                enemy_velocity.linear_velocity =
                    enemy_velocity.linear_velocity.lerp(*speed * dir, 0.5);
            }
            FleeAction::Teleport { distance, chance } => {
                if random < *chance {
                    enemy_transform.translation =
                        enemy_transform.translation + (*distance * wiggled_dir).extend(0.0);
                }
            }
        }
    }
}

fn attack_player(
    mut q_enemies: Query<(&mut Attack, &Transform), Without<Character>>,
    mut q_player: Query<(&Transform, &mut Character, &mut StatusEffects)>,
    time: Res<Time>,
) {
    let (player_transform, mut player_character, mut player_status_effects) =
        q_player.single_mut().expect("No Player Object");
    let player_translation = player_transform.translation.truncate();
    for (mut attack, enemy_transform) in q_enemies.iter_mut() {
        attack.cooldown.tick(time.delta());
        let enemy_translation = enemy_transform.translation.truncate();
        let distance = (enemy_translation - player_translation).length();
        if distance <= attack.radius && attack.cooldown.is_finished() {
            player_character.take_damage();
            if let Some(inflicts_status) = attack.inflicts_status {
                player_status_effects.add_effect(inflicts_status);
            }
            attack.cooldown.reset();
        }
    }
}

fn stalk_player(
    mut q_enemies: Query<(&mut Stalk, &Transform), Without<Character>>,
    mut q_player: Query<(&Transform, &mut StatusEffects)>,
) {
    let (player_transform, mut player_status_effects) =
        q_player.single_mut().expect("No Player Object");
    let player_translation = player_transform.translation.truncate();
    let is_stalked = q_enemies.iter_mut().any(|(stalk, enemy_transform)| {
        let enemy_translation = enemy_transform.translation.truncate();
        let distance = (enemy_translation - player_translation).length();
        distance <= stalk.radius
    });
    if is_stalked {
        player_status_effects.add_effect(StatusEffect::Stalked);
    } else {
        player_status_effects.remove_effect(StatusEffect::Stalked);
    }
}

fn apply_velocity(time: Res<Time>, mut q_enemies: Query<(&mut Transform, &Velocity), With<Enemy>>) {
    let dt = time.delta_secs();
    for (mut trans, vel) in q_enemies.iter_mut() {
        trans.translation.x += vel.linear_velocity.x * dt;
        trans.translation.y += vel.linear_velocity.y * dt;
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(
        Update,
        (
            teleport,
            track_player,
            flee_light,
            apply_velocity,
            stalk_player,
            attack_player,
        )
            .chain(),
    );
}
