use bevy::prelude::*;

use crate::character_controls::{Character, STARTING_HEALTH, StatusEffect, StatusEffects};

pub const UI_HEIGHT: f32 = 200.0;
pub const HEALTH_TEXT: &str = "HEALTH: ";
pub const STATUS_EFFECT_TEXT: &str = "EFFECTS: ";

#[derive(Component)]
pub struct UI;

fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("UI"),
            UI,
            Node {
                bottom: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(20.0)),
                ..Default::default()
            },
        ))
        .with_children(|p| {
            p.spawn((
                Name::new("Health Text"),
                Node {
                    margin: UiRect::horizontal(Val::Px(5.0)),
                    ..Default::default()
                },
            ))
            .with_child((
                Text::new(HEALTH_TEXT),
                TextFont {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            p.spawn((
                Name::new("Health Amount"),
                Node {
                    margin: UiRect::horizontal(Val::Px(5.0)),
                    ..Default::default()
                },
            ))
            .with_child((
                Text::new(STARTING_HEALTH.to_string()),
                TextFont {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            p.spawn((
                Name::new("Status Effect Text"),
                Node {
                    margin: UiRect::horizontal(Val::Px(5.0)),
                    ..Default::default()
                },
            ))
            .with_child((
                Text::new(STATUS_EFFECT_TEXT),
                TextFont {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            p.spawn((
                Name::new("Status Effects"),
                Node {
                    margin: UiRect::horizontal(Val::Px(5.0)),
                    ..Default::default()
                },
            ))
            .with_child((
                Text::new("None"),
                TextFont {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn update_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_player: Query<&Character>,
    q_status_effects: Query<&StatusEffects>,
) {
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, create_ui);
}
