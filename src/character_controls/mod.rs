use crate::{
    character_controls::flashlight::{
        Flashlight, FlashlightState, FlashlightToggle, FlashlightToggleState,
    },
    collision::Collider,
    dialog::DialogOnClose,
    items::CollectedItems,
    light::{CheckInLight, IgnoreInLightCheckLight},
    menu::Playing,
    room::Movable,
    sanity::Sanity,
    win::{CurrentState, GameState, SoundHandle},
};
use bevy::prelude::*;
use bevy::{platform::collections::HashSet, time::Stopwatch};
use bevy_kira_audio::{
    Audio, AudioControl, AudioInstance, AudioSource, AudioTween, SpatialAudioReceiver,
};
use bevy_lit::prelude::*;

pub mod flashlight;

const DEBUG_BRIGHTNESS: bool = true;
const BASE_MOVE_SPEED: f32 = 200.0;
const SLOWED_MULTIPLIER: f32 = 0.7;
const MOVE_SPEED_PERCENTAGE_REQUIRED_TO_ROTATE: f32 = 0.98;
const PLAYER_ASS_PATH: &str = "player_up.png";
pub const STARTING_HEALTH: i8 = 3;
// const PLAYER_SIZE: Option<Vec2> = Some(Vec2::new(64.0, 64.0));
// const ROOM_INSET: f32 = 4.0;

#[derive(Component)]
pub struct Character {
    pub health: i8,
    pub is_hurt: bool,
}

#[derive(Resource)]
pub struct Hurt(Handle<AudioSource>);

fn load_hurt_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Hurt(asset_server.load("sounds/punch.ogg")));
}

impl Character {
    pub fn take_damage(&mut self) {
        if self.health > 0 {
            self.health -= 1;
        }

        self.is_hurt = true;
    }

    pub fn heal(&mut self) {
        if self.health < STARTING_HEALTH {
            self.health += 1;
        }
    }
}

#[derive(Component)]
pub struct SpawnEnemies {
    pub stopwatch: Stopwatch,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum StatusEffect {
    Slowed,
    Stalked,
    Insane,
}

#[derive(Component)]
pub struct StatusEffects(pub HashSet<StatusEffect>);

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

fn play_hurt_sound(
    mut commands: Commands,
    audio: Res<Audio>,
    mut q_player: Query<&mut Character>,
    hurt: Res<Hurt>,
) {
    if let Ok(mut player) = q_player.single_mut()
        && player.is_hurt {
            player.is_hurt = false;
            commands.spawn(SoundHandle(
                audio.play(hurt.0.clone()).with_volume(-3.1).handle(),
            ));
        }
}

#[derive(Component, Default)]
pub struct Velocity {
    pub linear_velocity: Vec2,
}

fn get_speed(status_effects: &StatusEffects) -> f32 {
    if status_effects.0.contains(&StatusEffect::Slowed) {
        BASE_MOVE_SPEED * SLOWED_MULTIPLIER
    } else {
        BASE_MOVE_SPEED
    }
}

fn player_movement_input(
    audio: Res<Audio>,
    mut commands: Commands,
    inputs: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Velocity, &StatusEffects, &mut Walking), With<Character>>,
    q_walk_audio: Query<(Entity, &SoundHandle), With<WalkAudio>>,
    walk_sound: Res<Walk>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let (mut char_vel, status_effects, mut walk_state) =
        q_player.single_mut().expect("No Player Object");
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

    if dir != Vec2::ZERO && walk_state.0 == WalkState::Stopped {
        walk_state.0 = WalkState::Walking;
        commands.spawn((
            WalkAudio,
            SoundHandle(
                audio
                    .play(walk_sound.0.clone())
                    .with_volume(-20.0)
                    .looped()
                    .handle(),
            ),
        ));
    } else if dir == Vec2::ZERO {
        walk_state.0 = WalkState::Stopped;
        if let Ok((walk_audio, handle)) = q_walk_audio.single() {
            audio_instances
                .get_mut(&handle.0)
                .unwrap()
                .stop(AudioTween::default());
            commands.entity(walk_audio).despawn();
        }
    }

    let speed = get_speed(status_effects);
    char_vel.linear_velocity = char_vel
        .linear_velocity
        .lerp(dir.normalize_or_zero() * speed, 0.5);
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

pub(crate) fn apply_velocity(
    time: Res<Time>,
    mut q_player: Query<(&mut Transform, &Velocity), With<Character>>,
) {
    let dt = time.delta_secs();
    for (mut trans, vel) in q_player.iter_mut() {
        trans.translation.x += vel.linear_velocity.x * dt;
        trans.translation.y += vel.linear_velocity.y * dt;
        trans.translation.x = trans.translation.x.clamp(-7500.0, 7500.0);
        trans.translation.y = trans.translation.y.clamp(-3500.0, 3500.0);
    }
}

fn player_rotation_input(
    inputs: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Transform, &Velocity, &StatusEffects), With<Character>>,
) {
    let (mut trans, velocity, status_effects) = q_player.single_mut().expect("No Player Object");

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
        && velocity.linear_velocity.length()
            > get_speed(status_effects) * MOVE_SPEED_PERCENTAGE_REQUIRED_TO_ROTATE
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
            Camera2d,
            Lighting2dSettings {
                penetration: PenetrationSettings {
                    max: 30.0,
                    intensity: 1.0,
                    falloff: 1.0,
                    sample_directions: 16,
                    sample_steps: 8,
                },
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
                is_hurt: false,
            },
            // Mesh2d(meshes.add(Rectangle::new(45.0, 45.0))),
            // LightOccluder2d {
            //     occluder_mask: asset_server.load(PLAYER_ASS_PATH),
            // },
            (
                Sanity::default(),
                CheckInLight(32.0),
                StatusEffects(HashSet::new()),
                CollectedItems(HashSet::new()),
                Movable,
                Velocity::default(),
                Walking(WalkState::Stopped),
                Collider::square(45.0),
            ),
            Sprite {
                image: asset_server.load(PLAYER_ASS_PATH),
                custom_size: Some(Vec2::splat(45.0)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::Z * 0.0),
            IgnoreInLightCheckLight,
            PointLight2d {
                inner_radius: 0.0,
                outer_radius: 48.0,
                intensity: 0.1,
                falloff: 4.0,
                ..default()
            },
            CurrentState(GameState::Collecting),
            SpawnEnemies {
                stopwatch: Stopwatch::new(),
            },
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
                FlashlightToggle(FlashlightToggleState::Toggled),
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

    app.add_systems(
        Startup,
        (
            setup, /*load_profiles*/
            load_walk_sound,
            load_hurt_sound,
        ),
    );
    app.add_systems(
        Update,
        player_movement_input.run_if(resource_exists::<Playing>),
    );
    app.add_systems(
        PostUpdate,
        (
            apply_velocity,
            player_rotation_input,
            camera_follow_player,
            play_hurt_sound,
        )
            .chain(),
    );
}
