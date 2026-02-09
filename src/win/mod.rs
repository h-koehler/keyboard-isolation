use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{
    Audio, AudioControl, AudioEasing, AudioInstance, AudioSource, AudioTween, SpatialAudioEmitter,
    SpatialRadius,
};

use crate::{
    character_controls::Character,
    checkpoint::TimeTilNextPlay,
    dialog::show_dialog_on_condition,
    items::{CollectedItems, Item},
    menu::Playing,
};

pub const INTERACT_DIST: f32 = 400.0;
pub const SHIP_DURATION: f32 = 5.0;

#[derive(Component)]
pub struct RescuePoint;

#[derive(Component)]
pub struct CurrentState(pub GameState);

#[derive(PartialEq, Eq)]
pub enum GameState {
    Collecting,
    Collected,
    Finished,
}

#[derive(Component)]
pub struct SignalSent(pub SignalStatus);

#[derive(PartialEq, Eq)]
pub enum SignalStatus {
    NotSent,
    Sent,
}

fn rescue_point() -> impl Bundle {
    (
        Name::new("Rescue Point"),
        RescuePoint,
        SignalSent(SignalStatus::NotSent),
        SpatialAudioEmitter { instances: vec![] },
        SpatialRadius { radius: 5000.0 },
        TimeTilNextPlay(Timer::from_seconds(SHIP_DURATION, TimerMode::Repeating)),
    )
}

fn play_audio(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    q_game_state: Query<&CurrentState>,
    mut q_checkpoint: Query<(&mut TimeTilNextPlay, &mut SpatialAudioEmitter), With<RescuePoint>>,
    audio: Res<Audio>,
) {
    if let Ok(game_state) = q_game_state.single()
        && game_state.0 == GameState::Collected
    {
        let delta = time.delta_secs();
        for (mut timer, mut spatial_audio) in q_checkpoint.iter_mut() {
            timer.0.tick(Duration::from_secs_f32(delta));
            if timer.0.just_finished() {
                let ship_audio = audio
                    .play(asset_server.load("sounds/morse_SOS.ogg"))
                    .with_volume(-10.)
                    .fade_in(AudioTween::new(
                        Duration::from_millis(50),
                        AudioEasing::OutPowi(2),
                    ))
                    .handle();
                spatial_audio.instances.push(ship_audio);
            }
        }
    }
}

fn parts_collected(
    commands: Commands,
    asset_server: Res<AssetServer>,
    q_collected_items: Query<&CollectedItems>,
    mut q_game_state: Query<&mut CurrentState>,
) {
    let mut num_parts = 0;
    if let Ok(collected_items) = q_collected_items.single() {
        for item in collected_items.0.iter() {
            match item {
                Item::Antenna => num_parts += 1,
                Item::Chip => num_parts += 1,
                Item::Plate => num_parts += 1,
                _ => {}
            }
        }
    }

    if let Ok(mut game_state) = q_game_state.single_mut()
        && num_parts == 3
        && game_state.0 == GameState::Collecting
    {
        game_state.0 = GameState::Collected;
        show_dialog_on_condition(
            commands,
            asset_server,
            "I think I have all of the parts now! I need to go back to the ship!",
        );
    }
}

fn win(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_rescue_point: Query<&mut Transform, With<RescuePoint>>,
    mut q_player: Query<&Transform, (With<Character>, Without<RescuePoint>)>,
    mut q_game_state: Query<&mut CurrentState>,
) {
    let player_transform = q_player.single_mut().expect("No Player Object");
    let player_translation = player_transform.translation.truncate();

    if let Ok(item_transform) = q_rescue_point.single_mut() {
        let item_translation = item_transform.translation.truncate();
        let difference = player_translation - item_translation;
        if let Ok(mut game_state) = q_game_state.single_mut()
            && difference.length() <= INTERACT_DIST
            && game_state.0 == GameState::Collected
        {
            game_state.0 = GameState::Finished;
            show_dialog_on_condition(
                commands,
                asset_server,
                "There's the signal! Hopefully someone receives it soon... (YOU WIN!)",
            );
        }
    }
}

#[derive(Resource)]
pub struct SignalSend(Handle<AudioSource>);

fn load_win_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SignalSend(asset_server.load("sounds/scifi alarm.ogg")));
}

#[derive(Component)]
pub struct SoundHandle(pub Handle<AudioInstance>);

fn play_win_sound(
    mut commands: Commands,
    mut q_signal_sent: Query<&mut SignalSent>,
    q_game_state: Query<&CurrentState>,
    win_sound: Res<SignalSend>,
    audio: Res<Audio>,
) {
    if let Ok(mut signal_sent) = q_signal_sent.single_mut()
        && let Ok(game_state) = q_game_state.single()
        && signal_sent.0 == SignalStatus::NotSent
        && game_state.0 == GameState::Finished
    {
        signal_sent.0 = SignalStatus::Sent;
        commands.spawn(SoundHandle(
            audio.play(win_sound.0.clone()).with_volume(-3.0).handle(),
        ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        rescue_point(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 3.0)),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, (setup, load_win_sound));
    app.add_systems(
        Update,
        (
            (parts_collected, win).run_if(resource_exists::<Playing>),
            play_win_sound,
            play_audio,
        ),
    );
}
