use crate::{
    character_controls::flashlight::Flashlight,
    items::CollectedItems,
    light::{CheckInLight, IgnoreInLightCheckLight},
    room::Movable,
};
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy_lit::prelude::*;

pub mod flashlight;

const DEBUG_BRIGHTNESS: bool = false;
const MOVE_SPEED: f32 = 200.0;
const MOVE_SPEED_PERCENTAGE_REQUIRED_TO_ROTATE: f32 = 0.98;
const PLAYER_ASS_PATH: &str = "player_up.png";
pub const STARTING_HEALTH: i8 = 3;
// const PLAYER_SIZE: Option<Vec2> = Some(Vec2::new(64.0, 64.0));
// const ROOM_INSET: f32 = 4.0;

#[derive(Component)]
pub struct Character {
    pub health: i8,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum StatusEffect {
    Slowed,
    Blind,
    Bloodied,
}

#[derive(Component)]
pub struct StatusEffects(HashSet<StatusEffect>);

impl StatusEffects {
    pub fn add_effect(&mut self, status_effect: StatusEffect) {
        self.0.insert(status_effect);
    }

    pub fn remove_effect(&mut self, status_effect: StatusEffect) {
        self.0.remove(&status_effect);
    }

    pub fn iter(&self) -> impl Iterator<Item = StatusEffect> {
        self.0.iter().copied()
    }
}

#[derive(Component, Default)]
pub struct Velocity {
    pub linear_velocity: Vec2,
}

fn player_movement_input(
    inputs: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<&mut Velocity, With<Character>>,
) {
    let mut char_vel = q_player.single_mut().expect("No Player Object");
    let mut dir = Vec2::ZERO;
    if inputs.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if inputs.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }
    if inputs.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if inputs.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    char_vel.linear_velocity = char_vel
        .linear_velocity
        .lerp(dir.normalize_or_zero() * MOVE_SPEED, 0.5);
}

fn apply_velocity(
    time: Res<Time>,
    mut q_player: Query<(&mut Transform, &Velocity), With<Character>>,
) {
    let dt = time.delta_secs();
    for (mut trans, vel) in q_player.iter_mut() {
        trans.translation.x += vel.linear_velocity.x * dt;
        trans.translation.y += vel.linear_velocity.y * dt;
    }
}

fn player_rotation_input(
    inputs: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Transform, &Velocity), With<Character>>,
) {
    let (mut trans, velocity) = q_player.single_mut().expect("No Player Object");

    let mut dir = Vec2::ZERO;
    if inputs.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if inputs.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    if inputs.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if inputs.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }

    if dir.length() < f32::EPSILON
        && velocity.linear_velocity.length() > MOVE_SPEED * MOVE_SPEED_PERCENTAGE_REQUIRED_TO_ROTATE
    {
        dir = velocity.linear_velocity;
    }

    if dir.length() > f32::EPSILON {
        trans.rotation = trans
            .rotation
            .lerp(Quat::from_axis_angle(Vec3::Z, Vec2::X.angle_to(dir)), 0.1);
    }
}

fn take_damage(mut q_player: Query<&mut Character>) {
    let mut player = q_player.single_mut().expect("No Player Object");
    if player.health > 0 {
        player.health -= 1;
    } else {
        println!("Can't take any more damage.")
    }
}

fn heal(mut q_player: Query<&mut Character>) {
    let mut player = q_player.single_mut().expect("No Player Object");
    if player.health < STARTING_HEALTH {
        player.health += 1;
    } else {
        println!("Can't heal any more lives.")
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d::default(),
        Lighting2dSettings {
            ..Default::default()
        },
        AmbientLight2d {
            intensity: if DEBUG_BRIGHTNESS { 0.1 } else { 0.0 }, // More like darkness amiright,
            ..Default::default()
        },
    ));

    commands
        .spawn((
            Name::new("Character"),
            Character {
                health: STARTING_HEALTH,
            },
            StatusEffects(HashSet::new()),
            CollectedItems(HashSet::new()),
            Movable,
            CheckInLight(1.0),
            Velocity::default(),
            Sprite {
                image: asset_server.load(PLAYER_ASS_PATH),
                custom_size: Some(Vec2::splat(45.0)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::Z * 3.0),
            IgnoreInLightCheckLight,
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
                Flashlight {
                    battery: 20.0,
                    max_charge: 20.0,
                },
                SpotLight2d {
                    intensity: 0.0,
                    outer_radius: 1024.0,
                    outer_angle: 25.0,
                    ..default()
                },
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

fn camera_follow_player(
    mut q_cam: Query<&mut Transform, With<Camera2d>>,
    q_player: Query<(&Transform, &Velocity), (With<Character>, Without<Camera2d>)>,
) {
    let Ok((player_trans, player_vel)) = q_player.single() else {
        return;
    };
    for mut trans in q_cam.iter_mut() {
        let lerpped = trans.translation.lerp(
            player_trans.translation + player_vel.linear_velocity.extend(0.0) * 0.1,
            0.1,
        );
        trans.translation = Vec3::new(lerpped.x, lerpped.y, trans.translation.z);
    }
}

pub(super) fn register(app: &mut App) {
    flashlight::register(app);

    app.add_systems(Startup, (setup /*load_profiles*/,));
    app.add_systems(Update, player_movement_input);
    app.add_systems(
        PostUpdate,
        (apply_velocity, player_rotation_input, camera_follow_player).chain(),
    );
}
