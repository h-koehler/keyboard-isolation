use crate::{
    animation::AnimateSprite,
    animation::AnimationState,
    anim_clips::PLAYER_CLIPS,
    character_controls::flashlight::{Flashlight, FlashlightState},
    collision::Collider,
    dialog::DialogOnClose,
    items::CollectedItems,
    light::{CheckInLight, IgnoreInLightCheckLight},
    menu::Playing,
    room::Movable,
    assets::{LoadAssetsSet, load_atlas},
    sanity::Sanity,
    win::{CurrentState, GameState},
};
use bevy::prelude::*;
use bevy::{platform::collections::HashSet, time::Stopwatch};
use bevy_kira_audio::SpatialAudioReceiver;
use bevy_lit::prelude::*;

pub mod flashlight;

const DEBUG_BRIGHTNESS: bool = false;
const BASE_MOVE_SPEED: f32 = 200.0;
const SLOWED_MULTIPLIER: f32 = 0.7;
const MOVE_SPEED_PERCENTAGE_REQUIRED_TO_ROTATE: f32 = 0.98;
pub const STARTING_HEALTH: i8 = 3;

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

#[derive(Resource)]
pub struct PlayerAsset {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
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
    inputs: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Velocity, &mut AnimationState, &StatusEffects), With<Character>>,
) {
    let (mut vel, mut anim,status_effects) = q_player.single_mut().expect("No Player Object");
    let mut dir = Vec2::ZERO;
    if inputs.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if inputs.pressed(KeyCode::KeyD) { dir.x += 1.0; }
    if inputs.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if inputs.pressed(KeyCode::KeyS) { dir.y -= 1.0; }

    // 0 = idle/pause, 1 = walk (example mapping)
    if dir == Vec2::ZERO {
        anim.set_anim_state(0);
    } else {
        anim.set_anim_state(1);
    }
    let speed = get_speed(status_effects);
    vel.linear_velocity = vel
        .linear_velocity
        .lerp(dir.normalize_or_zero() * speed, 0.5);
}

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
        let angle = Vec2::X.angle_to(dir) + std::f32::consts::FRAC_PI_2; // rotate left 90°
        trans.rotation = trans.rotation.lerp(Quat::from_rotation_z(angle), 0.1);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, player_asset: Res<PlayerAsset>) {
    commands
        .spawn((
            Camera2d::default(),
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
            AnimateSprite { default_fps: 10, anim_state: 0, clips: PLAYER_CLIPS },
            Name::new("Character"),
            DialogOnClose("It's amazing I survived the crash...".into()),
            Character {
                health: STARTING_HEALTH,
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
                Collider::square(45.0),
            ),
            Sprite {
                image: player_asset.image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: player_asset.layout.clone(),
                    index: 0,
                }),
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

    app.add_systems(Startup, (setup.after(LoadAssetsSet/*load_profiles*/)));
    app.add_systems(
        Update,
        player_movement_input.run_if(resource_exists::<Playing>),
    );
    app.add_systems(
        PostUpdate,
        (apply_velocity, player_rotation_input, camera_follow_player).chain(),
    );
    load_atlas::<6, 64>(app, "astro-Sheet.png", |world, (texture, layout)| {
        world.insert_resource(PlayerAsset {
            image: texture,
            layout: layout,
        });
    });
}
