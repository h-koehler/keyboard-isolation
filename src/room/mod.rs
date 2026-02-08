use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{
    Audio, AudioControl, AudioEasing, AudioTween, SpatialAudioEmitter, SpatialRadius,
};
use bevy_lit::prelude::PointLight2d;

use crate::{
    animation::AnimateSprite,
    assets::{LoadAssetsSet, load_atlas},
    collision::room_objects::spawn_crash_site_objects,
    ui::UI_HEIGHT,
};

#[derive(Component)]
pub struct Movable;

#[derive(Resource)]
pub struct Ambiance(Handle<bevy::audio::AudioSource>);

fn load_ambiance(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Ambiance(asset_server.load("sounds/ambient_noise.ogg")));
}

#[derive(Component)]
struct FireOffset(f32);

pub const ROOM_HEIGHT: u32 = 700;
pub const ROOM_WIDTH: u32 = 1100;

fn fire(fire_asset: &FireAsset, audio: &Audio, asset_server: &AssetServer) -> impl Bundle {
    let fire_sound = audio
        .play(asset_server.load("sounds/fire.ogg"))
        .with_volume(-10.0)
        .fade_in(AudioTween::new(
            Duration::from_millis(50),
            AudioEasing::OutPowi(2),
        ))
        .loop_from(0.5)
        .loop_until(1.0)
        .handle();
    (
        AnimateSprite { fps: 10 },
        PointLight2d {
            inner_radius: 0.0,
            outer_radius: 400.0,
            cast_shadows: true,
            falloff: 5.0,
            intensity: 1.0,
            color: Srgba::hex("ffccaa").unwrap().into(),
            ..Default::default()
        },
        FireOffset(rand::random::<f32>() * 20.0),
        Name::new("Fire"),
        Sprite {
            image: fire_asset.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: fire_asset.layout.clone(),
                index: 0,
            }),
            ..Default::default()
        },
        SpatialAudioEmitter {
            instances: vec![fire_sound],
        },
        SpatialRadius { radius: 500.0 },
    )
}

fn setup_room(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fire_asset: Res<FireAsset>,
    ambiance: Res<Ambiance>,
    audio: Res<Audio>,
) {
    commands.spawn((
        Name::new("Background"),
        Sprite {
            // custom_size: Some(Vec2::new(ROOM_WIDTH as f32, ROOM_HEIGHT as f32)),
            image: asset_server.load("background.png"),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, UI_HEIGHT / 2.0, -10.0))
            .with_scale(Vec3::splat(4.0)),
        AudioPlayer::new(ambiance.0.clone()),
        PlaybackSettings {
            volume: bevy::audio::Volume::Linear(0.05),
            mode: bevy::audio::PlaybackMode::Loop,
            start_position: Some(Duration::from_secs_f32(0.1)),
            duration: Some(Duration::from_secs_f32(10.5)),
            ..Default::default()
        },
    ));

    commands.spawn((
        Transform::from_xyz(0.0, 100.0, 0.0),
        fire(&fire_asset, &audio, &asset_server),
    ));
    commands.spawn((
        Transform::from_xyz(100.0, 100.0, 0.0),
        fire(&fire_asset, &audio, &asset_server),
    ));
    commands.spawn((
        Transform::from_xyz(200.0, 400.0, 0.0),
        fire(&fire_asset, &audio, &asset_server),
    ));
    commands.spawn((
        Transform::from_xyz(100.0, -100.0, 0.0),
        fire(&fire_asset, &audio, &asset_server),
    ));
    spawn_crash_site_objects(&mut commands, &asset_server);
}

#[derive(Resource)]
pub struct FireAsset {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

fn flicker_fire(time: Res<Time>, mut q_fire: Query<(&FireOffset, &mut PointLight2d)>) {
    for (offset, mut light) in q_fire.iter_mut() {
        light.intensity = 0.25 + 0.25 * (offset.0 + time.elapsed_secs()).sin().abs();
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(
        Startup,
        (load_ambiance, setup_room.after(LoadAssetsSet)).chain(),
    )
    .add_systems(Update, flicker_fire);

    load_atlas::<7, 32>(app, "fire.png", |world, (texture, layout)| {
        world.insert_resource(FireAsset {
            image: texture,
            layout: layout,
        });
    });
}
