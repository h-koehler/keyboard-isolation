use bevy::prelude::*;
use bevy_lit::prelude::PointLight2d;

use crate::{
    animation::AnimateSprite,
    assets::{LoadAssetsSet, load_atlas},
    ui::UI_HEIGHT,
};

#[derive(Component)]
pub struct Movable;

pub const ROOM_HEIGHT: u32 = 700;
pub const ROOM_WIDTH: u32 = 1100;

fn fire(fire_asset: &FireAsset) -> impl Bundle {
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
            .with_scale(Vec3::splat(4.0)),
    ));

    commands.spawn((Transform::from_xyz(0.0, 100.0, 0.0), fire(&fire_asset)));
}

#[derive(Resource)]
pub struct FireAsset {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, setup_room.after(LoadAssetsSet));

    load_atlas::<7, 32>(app, "fire.png", |world, (texture, layout)| {
        world.insert_resource(FireAsset {
            image: texture,
            layout: layout,
        });
    });
}
