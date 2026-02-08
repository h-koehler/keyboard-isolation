use crate::{
    character_controls::flashlight::{Flashlight, FlashlightState},
    collision::Collider,
    dialog::DialogOnClose,
    items::CollectedItems,
    light::{CheckInLight, IgnoreInLightCheckLight},
    room::Movable,
    sanity::Sanity,
    win::{CurrentState, GameState},
    y_sort::YSort,
};
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy_kira_audio::SpatialAudioReceiver;
use bevy_lit::prelude::*;

pub mod flashlight;

const DEBUG_BRIGHTNESS: bool = true;
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

impl Character {
    pub fn take_damage(&mut self) {
        if self.health > 0 {
            self.health -= 1;
        }
    }

    pub fn heal(&mut self) {
        if self.health < STARTING_HEALTH {
            self.health += 1;
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum StatusEffect {
    Slowed,
    Blind,
    Stalked,
    Insane,
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
    mut commands: Commands,
    inputs: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Velocity, &mut Walking), With<Character>>,
    q_walk_audio: Query<Entity, With<WalkAudio>>,
    walk_sound: Res<Walk>,
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

    if dir != Vec2::ZERO && char_vel.1.0 == WalkState::Stopped {
        char_vel.1.0 = WalkState::Walking;
        commands.spawn((
            WalkAudio,
            AudioPlayer::new(walk_sound.0.clone()),
            PlaybackSettings {
                volume: bevy::audio::Volume::Linear(0.1),
                mode: bevy::audio::PlaybackMode::Loop,
                ..Default::default()
            },
        ));
    } else if dir == Vec2::ZERO {
        char_vel.1.0 = WalkState::Stopped;
        if let Ok(walk_audio) = q_walk_audio.single() {
            commands.entity(walk_audio).despawn();
        }
    }

    char_vel.0.linear_velocity = char_vel
        .0
        .linear_velocity
        .lerp(dir.normalize_or_zero() * MOVE_SPEED, 0.5);
}

#[derive(Resource)]
pub struct Walk(Handle<AudioSource>);

#[derive(Component)]
pub struct WalkAudio;

#[derive(PartialEq, Eq)]
pub enum WalkState {
    Walking,
    Stopped,
}

#[derive(Component)]
pub struct Walking(WalkState);

fn load_walk_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Walk(asset_server.load("sounds/walking.ogg")));
}

// fn play_walk_sound(
//     q_player: Query<&Velocity, (With<Character>, Without<Camera2d>)>,
//     walk_sound: Res<Walk>,
// ) {
//     let char_vel = q_player.single().expect("No Player Object");
//     if char_vel.linear_velocity.normalize_or_zero() == Vec2::ZERO {
//         c
//     }
// }

pub(crate) fn apply_velocity(
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Camera2d::default(),
            Lighting2dSettings {
                ..Default::default()
            },
            AmbientLight2d {
                intensity: if DEBUG_BRIGHTNESS { 0.1 } else { 0.0 }, // More like darkness amiright,
                ..Default::default()
            },
        ))
        .insert(SpatialAudioReceiver);

    commands
        .spawn((
            Name::new("Character"),
            DialogOnClose("It's amazing I survived the crash...".into()),
            Character {
                health: STARTING_HEALTH,
            },
            (
                Sanity::default(),
                CheckInLight(32.0),
                StatusEffects(HashSet::new()),
                CollectedItems(HashSet::new()),
                Movable,
                Velocity::default(),
                Walking(WalkState::Stopped),
                Collider::square(45.0),
                YSort::default_layer(),
            ),
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
            CurrentState(GameState::Collecting),
        ))
        .with_children(|p| {
            p.spawn((
                Flashlight {
                    battery: 20.0,
                    max_charge: 20.0,
                    state: FlashlightState::Lost,
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

    app.add_systems(Startup, (setup /*load_profiles*/, load_walk_sound));
    app.add_systems(Update, player_movement_input);
    app.add_systems(
        PostUpdate,
        (
            apply_velocity,
            player_rotation_input,
            camera_follow_player,
            // play_walk_sound,
        )
            .chain(),
    );
}
