use bevy::prelude::*;

use crate::{
    character_controls::Character,
    dialog::show_dialog_on_condition,
    items::{CollectedItems, Item},
};

pub const INTERACT_DIST: f32 = 50.0;

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

fn rescue_point(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Rescue Point"),
        RescuePoint,
        Sprite {
            image: asset_server.load("rescue_point.png"),
            custom_size: Some(Vec2::splat(45.0)),
            ..Default::default()
        },
    )
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

    if let Ok(mut game_state) = q_game_state.single_mut() {
        if num_parts == 3 && game_state.0 == GameState::Collecting {
            game_state.0 = GameState::Collected;
            show_dialog_on_condition(
                commands,
                asset_server,
                "I think I have all of the parts now! Now where should I put them together?",
            );
        }
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
        if let Ok(mut game_state) = q_game_state.single_mut() {
            if difference.length() <= INTERACT_DIST && game_state.0 == GameState::Collected {
                game_state.0 = GameState::Finished;
                show_dialog_on_condition(
                    commands,
                    asset_server,
                    "There's the signal! Hopefully someone receives it soon...",
                );
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        rescue_point(&asset_server),
        Transform::from_translation(Vec3::new(2000.0, 0.0, 3.0)),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, (parts_collected, win));
}
