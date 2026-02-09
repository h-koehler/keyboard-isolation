use bevy::prelude::*;

use crate::{
    character_controls::{Character, STARTING_HEALTH, StatusEffect, StatusEffects},
    dialog::DialogOnClose,
    items::{CollectedItems, Item},
};

pub const UI_HEIGHT: f32 = 200.0;
pub const HEALTH_TEXT: &str = "HEALTH: ";
pub const STATUS_EFFECT_TEXT: &str = "EFFECTS: ";

#[derive(Component)]
pub struct HealthUI;

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct StatusUI;

#[derive(Component)]
pub struct ItemsUI;

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
                    margin: UiRect::left(Val::Px(40.0)),
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
                    margin: UiRect::left(Val::Px(150.0)),
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
                    width: Val::Px(150.0),
                    height: Val::Px(32.0),
                    ..Default::default()
                },
            ));
            p.spawn((
                Name::new("Beacon Components"),
                ItemsUI,
                Node {
                    margin: UiRect::left(Val::Px(150.0)),
                    width: Val::Px(350.0),
                    height: Val::Px(32.0),
                    ..Default::default()
                },
            ))
            .with_child((
                Text::new("BEACON COMPONENTS: 0 OF 3"),
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
    q_player: Query<(Entity, &Character, Has<Dead>)>,
    q_status_effects: Query<&StatusEffects>,
    q_collected_items: Query<&CollectedItems>,
    q_health_ui: Query<Entity, With<HealthUI>>,
    q_status_ui: Query<Entity, With<StatusUI>>,
    q_items_ui: Query<Entity, With<ItemsUI>>,
) {
    if let Ok((player_entity, player_character, is_dead)) = q_player.single() {
        if player_character.health == 0 && !is_dead {
            commands
                .entity(player_entity)
                .insert((Dead, DialogOnClose("Oof ouchy I'm dead.".into())));
        }
        if let Ok(health_ui) = q_health_ui.single() {
            commands.entity(health_ui).despawn_children();
            commands.entity(health_ui).with_child((
                Text::new(player_character.health.to_string()),
                TextFont {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        }

        if let Ok(status_effects) = q_status_effects.single() {
            let status_icons = status_effects
                .iter()
                .map(|status_effect| match status_effect {
                    StatusEffect::Slowed => "cripple_icon.png",
                    StatusEffect::Stalked => "insanity_icon.png",
                    StatusEffect::Insane => "insanity_icon.png",
                });
            if let Ok(status_ui) = q_status_ui.single() {
                commands.entity(status_ui).despawn_children();
                commands.entity(status_ui).with_children(|p| {
                    for status_icon in status_icons {
                        p.spawn((
                            ImageNode {
                                image: asset_server.load(status_icon),
                                ..Default::default()
                            },
                            Node {
                                margin: UiRect::left(Val::Px(4.0)),
                                width: Val::Px(32.0),
                                height: Val::Px(32.0),
                                ..Default::default()
                            },
                        ));
                    }
                });
            }
        }

        if let Ok(collected_items) = q_collected_items.single() {
            let count: u32 = collected_items
                .iter()
                .map(|item| match item {
                    Item::Antenna | Item::Chip | Item::Plate => 1,
                    _ => 0,
                })
                .sum();
            if let Ok(items_ui) = q_items_ui.single() {
                commands.entity(items_ui).despawn_children();
                commands.entity(items_ui).with_child((
                    Text::new(format!("BEACON COMPONENTS: {} OF 3", count)),
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
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, create_ui);
    app.add_systems(PostUpdate, update_ui);
}
