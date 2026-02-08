use bevy::prelude::*;
use bevy_lit::prelude::PointLight2d;

use crate::{animation::AnimateSprite, ui::UI_HEIGHT};

#[derive(Component)]
pub struct Movable;

pub const ROOM_HEIGHT: u32 = 700;
pub const ROOM_WIDTH: u32 = 1100;

fn setup_room(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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
    ));

    // Load the sprite sheet using the `AssetServer`
    let texture = asset_server.load("fire.png");

    // The sprite sheet has 7 sprites arranged in a row, and they are all 32x32
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        AnimateSprite { fps: 10 },
        Transform::from_xyz(0.0, 100.0, 0.0),
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
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            ..Default::default()
        },
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, setup_room);
}
