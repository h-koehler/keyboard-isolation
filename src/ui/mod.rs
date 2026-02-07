use bevy::{color::palettes::css, prelude::*};

use crate::character_controls::Character;

pub const UI_HEIGHT: f32 = 200.0;
pub const LEVEL: &str = "LEVEL";

fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>, q_player: Query<&Character>) {
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
            for player in q_player.iter() {
                p.spawn((
                    Name::new("Health Text"),
                    Node {
                        margin: UiRect::horizontal(Val::Px(5.0)),
                        ..Default::default()
                    },
                ))
                .with_child((
                    Text::new(LEVEL),
                    TextFont {
                        font: asset_server.load("fonts/default.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ));
                p.spawn((
                    Name::new("Health Amount"),
                    Node {
                        margin: UiRect::horizontal(Val::Px(5.0)),
                        ..Default::default()
                    },
                ))
                .with_child((
                    Text::new(player.health.to_string()),
                    TextFont {
                        font: asset_server.load("fonts/default.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ));
            }
        });
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, create_ui);
}
