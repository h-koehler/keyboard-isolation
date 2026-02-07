use bevy::{color::palettes::css, prelude::*};

use crate::character_controls::Character;

#[derive(Component)]
pub struct Dialog(String);

impl From<&str> for Dialog {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[derive(Component)]
pub struct DialogOnClose(pub Dialog);

#[derive(Component)]
pub struct DialogNode;

fn show_dialog_on_close(
    mut commands: Commands,
    q_player: Query<&Transform, With<Character>>,
    q_close: Query<(&Transform, &DialogOnClose, Entity)>,
    asset_server: Res<AssetServer>,
) {
    let Ok(trans) = q_player.single() else {
        return;
    };

    if let Some((_, dialog, ent)) = q_close
        .iter()
        .find(|(t, _, _)| t.translation.distance(trans.translation) < 100.0)
    {
        commands.entity(ent).remove::<DialogOnClose>();
        commands
            .spawn((
                DialogNode,
                BorderColor::all(css::GOLD),
                BackgroundColor(Color::BLACK),
                Node {
                    top: Val::Px(600.0),
                    width: Val::Px(400.0),
                    min_height: Val::Px(300.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..Default::default()
                },
            ))
            .with_children(|p| {
                p.spawn((
                    Text::new(dialog.0.0.clone()),
                    TextFont {
                        font: asset_server.load("fonts/default.ttf"),
                        font_size: 52.0,
                        ..Default::default()
                    },
                ));
            });
    }
    // trans.translation
}

fn close_dialog_on_enter(
    mut commands: Commands,
    q_dialog: Query<Entity, With<DialogNode>>,
    inputs: Res<ButtonInput<KeyCode>>,
) {
    let Ok(dialog_node) = q_dialog.single() else {
        return;
    };

    if !(inputs.just_pressed(KeyCode::Enter) || inputs.just_pressed(KeyCode::NumpadEnter)) {
        return;
    }

    commands.entity(dialog_node).despawn();
}

pub(super) fn register(app: &mut App) {
    app.add_systems(
        Update,
        (close_dialog_on_enter, show_dialog_on_close).chain(),
    );
}
