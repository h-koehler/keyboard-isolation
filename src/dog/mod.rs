use bevy::prelude::*;

use crate::{
    character_controls::{Character, Velocity},
    enemies::Enemy,
    room::Movable,
};

#[derive(Clone, Copy)]
pub enum DogState {
    Lost,
    Collected,
}

#[derive(Component)]
pub struct Dog(DogState);

#[derive(Resource)]
struct Bark(Handle<AudioSource>);

impl Dog {
    pub fn collect_dog(&mut self) {
        self.0 = DogState::Collected;
    }
}

fn load_bark_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Bark(asset_server.load("sounds/alien dog.ogg")));
}

#[derive(Component)]
pub struct FollowPlayer {
    outer_radius: f32,
    inner_radius: f32,
    speed: f32,
}

fn dog(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Dog"),
        Dog(DogState::Lost),
        Movable,
        Velocity::default(),
        FollowPlayer {
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
    mut commands: Commands,
    mut q_dog: Query<(&FollowPlayer, &mut Transform, &mut Velocity, &mut Dog)>,
    mut q_player: Query<&Transform, (With<Character>, Without<Enemy>, Without<Dog>)>,
    bark: Res<Bark>, // profiles: Res<PlayerProfiles>,
) {
    let player_transform = q_player.single_mut().expect("No Player Object");
    let player_translation = player_transform.translation.truncate();

    for (track_player, dog_transform, mut dog_velocity, mut dog) in q_dog.iter_mut() {
        let dog_translation = dog_transform.translation.truncate();
        let difference = player_translation - dog_translation;

        if difference.length() <= track_player.outer_radius
            && difference.length() >= track_player.inner_radius
        {
            match dog.0 {
                DogState::Lost => {
                    dog.collect_dog();
                    commands.spawn((
                        AudioPlayer::new(bark.0.clone()),
                        PlaybackSettings {
                            volume: bevy::audio::Volume::Linear(0.5),
                            ..Default::default()
                        },
                    ));
                }
                DogState::Collected => {}
            }

            let dir = difference.normalize_or_zero();

            dog_velocity.linear_velocity = dog_velocity
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
            dog_velocity.linear_velocity = dog_velocity.linear_velocity.lerp(Vec2::ZERO, 0.5);
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
    app.add_systems(Startup, (setup, load_bark_sound));
    app.add_systems(Update, track_player);
    app.add_systems(PostUpdate, apply_velocity);
}
