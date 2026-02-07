use bevy::prelude::*;

use crate::character_controls::{Character, STARTING_HEALTH, StatusEffect, StatusEffects};

pub const UI_HEIGHT: f32 = 200.0;
pub const HEALTH_TEXT: &str = "HEALTH: ";
pub const STATUS_EFFECT_TEXT: &str = "EFFECTS: ";

#[derive(Component)]
pub struct HealthUI;

#[derive(Component)]
pub struct StatusUI;

fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("UI"),
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
                HealthUI,
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
                StatusUI,
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
    q_health_ui: Query<Entity, With<HealthUI>>,
    q_status_ui: Query<Entity, With<StatusUI>>,
) {
    if let Ok(player) = q_player.single() {
        if let Ok(health_ui) = q_health_ui.single() {
            commands.entity(health_ui).despawn_children();
            commands.entity(health_ui).with_child((
                Text::new(player.health.to_string()),
                TextFont {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        }

        let mut status_effects_text = String::from("");

        if let Ok(status_effects) = q_status_effects.single() {
            for status_effect in status_effects.iter() {
                let status_effect_text;
                match status_effect {
                    StatusEffect::Blind => status_effect_text = "Blind, ",
                    StatusEffect::Slowed => status_effect_text = "Slowed, ",
                }
                status_effects_text.push_str(status_effect_text);
            }
        }

        if let Ok(status_ui) = q_status_ui.single() {
            commands.entity(status_ui).despawn_children();
            commands.entity(status_ui).with_child((
                Text::new(status_effects_text),
                TextFont {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, create_ui);
}
