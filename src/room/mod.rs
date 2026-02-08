use bevy::prelude::*;

use crate::{ui::UI_HEIGHT};

#[derive(Component)]
pub struct Movable;


pub const ROOM_HEIGHT: u32 = 700;
pub const ROOM_WIDTH: u32 = 1100;


fn setup_room(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn((
        Name::new("Background"),
        Sprite {
            // custom_size: Some(Vec2::new(ROOM_WIDTH as f32, ROOM_HEIGHT as f32)),
            image: asset_server.load("background.png"),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, UI_HEIGHT / 2.0, -10.0)).with_scale(Vec3::splat(4.0)),
    ));

}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, setup_room);
}
