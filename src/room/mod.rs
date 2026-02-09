use bevy::prelude::*;
use bevy_lit::prelude::PointLight2d;

use crate::{
    animation::AnimateSprite,
    anim_clips::fire_clips,
    assets::{LoadAssetsSet, load_atlas},
    collision::room_objects::spawn_crash_site_objects,
    ui::UI_HEIGHT,
};

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
struct FireOffset(f32);

pub const ROOM_HEIGHT: u32 = 700;
pub const ROOM_WIDTH: u32 = 1100;

fn fire(fire_asset: &FireAsset) -> impl Bundle {
    (
        AnimateSprite { default_fps: 10, anim_state: 0, clips: fire_clips },
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
    )
}

fn setup_room(mut commands: Commands, asset_server: Res<AssetServer>, fire_asset: Res<FireAsset>) {
    commands.spawn((
        Name::new("Background"),
        Sprite {
            // custom_size: Some(Vec2::new(ROOM_WIDTH as f32, ROOM_HEIGHT as f32)),
            image: asset_server.load("background.png"),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, UI_HEIGHT / 2.0, -10.0))
            .with_scale(Vec3::splat(2.0)),
    ));

    // [0,700],[0,200]
    commands.spawn((
        Transform::from_xyz(0.0, 100.0, 2.0).with_scale(Vec3::splat(5.0)),
        fire(&fire_asset),
    ));
    commands.spawn((
        Transform::from_xyz(50.0, 11.0, 2.0).with_scale(Vec3::splat(4.0)),
        fire(&fire_asset),
    ));
    commands.spawn((
        Transform::from_xyz(300.0, 150.0, 2.0).with_scale(Vec3::splat(3.0)),
        fire(&fire_asset),
    ));
    commands.spawn((
        Transform::from_xyz(350.0, 130.0, 2.0).with_scale(Vec3::splat(2.0)),
        fire(&fire_asset),
    ));
    commands.spawn((
        Transform::from_xyz(250.0, 140.0, 2.0).with_scale(Vec3::splat(2.0)),
        fire(&fire_asset),
    ));
    commands.spawn((
        Transform::from_xyz(320.0, 100.0, 2.0).with_scale(Vec3::splat(2.0)),
        fire(&fire_asset),
    ));

    commands.spawn((
        Transform::from_xyz(700.0, 50.0, 2.0).with_scale(Vec3::splat(4.5)),
        fire(&fire_asset),
    ));
    commands.spawn((
        Transform::from_xyz(450.0, 30.0, 2.0).with_scale(Vec3::splat(2.0)),
        fire(&fire_asset),
    ));
    commands.spawn((
        Transform::from_xyz(800.0, 138.0, 2.0).with_scale(Vec3::splat(1.0)),
        fire(&fire_asset),
    ));

    commands.spawn((Transform::from_xyz(100.0, 100.0, 2.0), fire(&fire_asset)));
    commands.spawn((Transform::from_xyz(200.0, 400.0, 2.0), fire(&fire_asset)));
    commands.spawn((Transform::from_xyz(100.0, -100.0, 2.0), fire(&fire_asset)));
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
    app.add_systems(Startup, setup_room.after(LoadAssetsSet))
        .add_systems(Update, flicker_fire);

    load_atlas::<7, 32>(app, "fire.png", |world, (texture, layout)| {
        world.insert_resource(FireAsset {
            image: texture,
            layout: layout,
        });
    });
}
