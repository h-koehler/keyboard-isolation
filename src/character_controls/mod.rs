use std::f32::consts::PI;

use crate::{
    room::{Movable, ROOM_HEIGHT, ROOM_WIDTH},
    ui::UI_HEIGHT,
};
use bevy::prelude::*;
use bevy_lit::prelude::*;

const MOVE_SPEED: f32 = 200.0;
const VELOCITY_CHANGE: f32 = 1.0;
const PLAYER_ASS_PATH: &str = "player_up.png";
const PLAYER_SIZE: Option<Vec2> = Some(Vec2::new(64.0, 64.0));
const ROOM_INSET: f32 = 4.0;

#[derive(Component)]
pub struct Character;

#[derive(Component, Default)]
pub struct Velocity {
    pub linear_velocity: Vec2,
}

fn player_input(
    inputs: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Transform, &mut Velocity), With<Character>>,
) {
    let (mut trans, mut char_vel) = q_player.single_mut().expect("No Player Object");
    let mut dir = Vec2::ZERO;

    if inputs.pressed(KeyCode::KeyA) {
        dir.x -= VELOCITY_CHANGE;
        trans.rotation = Quat::from_axis_angle(Vec3::Z, PI / 2.0);
    }
    if inputs.pressed(KeyCode::KeyD) {
        dir.x += VELOCITY_CHANGE;
        trans.rotation = Quat::from_axis_angle(Vec3::Z, -PI / 2.0);
    }
    if inputs.pressed(KeyCode::KeyW) {
        dir.y += VELOCITY_CHANGE;
        trans.rotation = Quat::from_axis_angle(Vec3::Z, 0.0);
    }
    if inputs.pressed(KeyCode::KeyS) {
        dir.y -= VELOCITY_CHANGE;
        trans.rotation = Quat::from_axis_angle(Vec3::Z, PI);
    }

    char_vel.linear_velocity = char_vel
        .linear_velocity
        .lerp(dir.normalize_or_zero() * MOVE_SPEED, 0.5);
}

fn apply_velocity(
    time: Res<Time>,
    mut q_player: Query<(&mut Transform, &Velocity, Has<Character>)>,
) {
    let dt = time.delta_secs();
    for (mut trans, vel, is_player) in q_player.iter_mut() {
        trans.translation.x += vel.linear_velocity.x * dt;
        trans.translation.y += vel.linear_velocity.y * dt;

        if !is_player {
            continue;
        }

        let half_width = ROOM_WIDTH as f32 / 2.0;
        let half_height = ROOM_HEIGHT as f32 / 2.0;

        let (half_player_width, half_player_height) = if let Some(size) = PLAYER_SIZE {
            (size.x * 0.5, size.y * 0.5)
        } else {
            (50.0, 50.0)
        };

        let min_x = -half_width + half_player_width + ROOM_INSET;
        let max_x = half_width - half_player_width - ROOM_INSET;
        let min_y = UI_HEIGHT / 2.0 + -half_height + half_player_height + ROOM_INSET;
        let max_y = UI_HEIGHT / 2.0 + half_height - half_player_height - ROOM_INSET;

        trans.translation.x = trans.translation.x.clamp(min_x, max_x);
        trans.translation.y = trans.translation.y.clamp(min_y, max_y);
    }
}

#[derive(Component)]
pub struct Flashlight;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d::default(),
        Lighting2dSettings {
            ..Default::default()
        },
        AmbientLight2d {
            intensity: 0.0,
            ..Default::default()
        },
    ));

    commands
        .spawn((
            Character,
            Movable,
            Velocity::default(),
            Sprite {
                image: asset_server.load(PLAYER_ASS_PATH),
                custom_size: Some(Vec2::splat(45.0)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::Z * 3.0),
            PointLight2d {
                inner_radius: 0.0,
                outer_radius: 48.0,
                intensity: 0.1,
                falloff: 4.0,
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn((
                Flashlight,
                SpotLight2d {
                    intensity: 1.0,
                    outer_radius: 1024.0,
                    outer_angle: 25.0,
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(90_f32.to_radians())),
            ));
        });
}

// #[derive(Resource)]
// struct PlayerProfiles {
//     left: Handle<Image>,
//     right: Handle<Image>,
//     up: Handle<Image>,
//     down: Handle<Image>,
// }

// fn load_profiles(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // commands.insert_resource(PlayerProfiles {
//     //     up: asset_server.load("player_up.png"),
//     //     down: asset_server.load("player.png"),
//     //     left: asset_server.load("player_left.png"),
//     //     right: asset_server.load("player_right.png"),
//     // });
// }

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, (setup /*load_profiles*/,));
    app.add_systems(Update, player_input);
    app.add_systems(PostUpdate, apply_velocity);
}
