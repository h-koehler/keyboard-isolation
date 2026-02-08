use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{SpatialRadius, prelude::*};
use bevy_lit::prelude::PointLight2d;

use crate::{
    character_controls::{Character, flashlight::Flashlight},
    dialog::{Dialog, DialogOnClose},
    sanity::body::{DeadSO, dead_body},
};

pub const CHECKPOINT_DURATION: f32 = 5.0;

#[derive(Component)]
#[require(Transform)]
pub struct Checkpoint;

#[derive(Component)]
pub struct CheckpointDone;

#[derive(Component, Default)]
pub struct CheckpointBlinking(f32);

#[derive(Component)]
pub struct TimeTilNextPlay(Timer);

#[derive(Resource)]
pub struct GetCheckpoint(Handle<bevy::audio::AudioSource>);

fn load_get_checkpoint_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GetCheckpoint(asset_server.load("sounds/achievement.ogg")));
}

fn on_add_checkpoint(mut commands: Commands, q_check: Query<Entity, Added<Checkpoint>>) {
    for e in q_check.iter() {
        commands.entity(e).insert((
            PointLight2d {
                inner_radius: 80.0,
                outer_radius: 120.0,
                cast_shadows: true,
                falloff: 5.0,
                intensity: 0.0,
                ..Default::default()
            },
            CheckpointBlinking(0.0),
            SpatialAudioEmitter { instances: vec![] },
            SpatialRadius { radius: 500.0 },
            TimeTilNextPlay(Timer::from_seconds(
                CHECKPOINT_DURATION,
                TimerMode::Repeating,
            )),
        ));
    }
}

fn play_audio(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut q_checkpoint: Query<
        (&mut TimeTilNextPlay, &mut SpatialAudioEmitter),
        With<CheckpointBlinking>,
    >,
    audio: Res<Audio>,
) {
    let delta = time.delta_secs();
    for (mut timer, mut spatial_audio) in q_checkpoint.iter_mut() {
        timer.0.tick(Duration::from_secs_f32(delta));
        if timer.0.just_finished() {
            let checkpoint_audio = audio
                .play(asset_server.load("sounds/morse_SOS.ogg"))
                .with_volume(-10.)
                .fade_in(AudioTween::new(
                    Duration::from_millis(50),
                    AudioEasing::OutPowi(2),
                ))
                .handle();
            spatial_audio.instances.push(checkpoint_audio);
        }
    }
}

const BLINK_INTERVAL_SECS: f32 = 5.0;

fn blink_checkpoint(
    time: Res<Time>,
    mut q_blinking: Query<(&mut PointLight2d, &mut CheckpointBlinking)>,
) {
    let delta = time.delta_secs();
    for (mut light, mut blinking) in q_blinking.iter_mut() {
        blinking.0 += delta;
        if blinking.0 >= BLINK_INTERVAL_SECS {
            blinking.0 = BLINK_INTERVAL_SECS - blinking.0;
            light.intensity = 0.5;
        } else {
            light.intensity = 0.0;
        }
    }
}

fn done_checkpoint(
    mut q_done: Query<&mut PointLight2d, Added<CheckpointDone>>,
    mut q_flashlight: Query<&mut Flashlight>,
) {
    for mut light in q_done.iter_mut() {
        light.intensity = 1.0;
        light.outer_radius = 320.0;

        if let Ok(mut flashlight) = q_flashlight.single_mut() {
            flashlight.battery = flashlight.max_charge;
        }
    }
}

fn checkpoint(asset_server: &AssetServer, text: impl Into<Dialog>) -> impl Bundle {
    (
        Checkpoint,
        CheckpointBlinking::default(),
        DialogOnClose(text.into()),
        Sprite {
            image: asset_server.load("checkpoint.png"),
            ..Default::default()
        },
    )
}

fn spawn_checkpoint(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 400.0, 0.0)),
        checkpoint(
            &asset_server,
            "A recharge station! I should be safe here. Maybe my friends are at other stations?",
        ),
    ));

    // commands.spawn((
    //     Transform::from_translation(Vec3::new(-400.0, 0.0, 0.0)),
    //     checkpoint(&asset_server, "2"),
    // ));

    // commands.spawn((
    //     Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)),
    //     checkpoint(&asset_server, "3"),
    // ));

    commands.spawn((
        Transform::from_translation(Vec3::new(450.0, 200.0, 0.0)),
        dead_body::<DeadSO>(asset_server, "dead_person_red.png"),
    ));
}

fn done_checkpoint_on_close(
    mut commands: Commands,
    q_player: Query<&Transform, With<Character>>,
    q_close: Query<(&Transform, Entity), With<CheckpointBlinking>>,
    get_checkpoint_sound: Res<GetCheckpoint>,
) {
    let Ok(trans) = q_player.single() else {
        return;
    };

    if let Some((_, ent)) = q_close
        .iter()
        .find(|(t, _)| t.translation.distance(trans.translation) < 100.0)
    {
        commands
            .entity(ent)
            .remove::<CheckpointBlinking>()
            .insert(CheckpointDone);
        commands.spawn((
            AudioPlayer::new(get_checkpoint_sound.0.clone()),
            PlaybackSettings {
                volume: bevy::audio::Volume::Linear(0.5),
                ..Default::default()
            },
        ));
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, (spawn_checkpoint, load_get_checkpoint_sound));
    app.add_systems(
        Update,
        (
            play_audio,
            on_add_checkpoint,
            blink_checkpoint,
            done_checkpoint_on_close,
            done_checkpoint,
        ),
    );
}
