use bevy::prelude::*;

use crate::character_controls::Character;

#[derive(Component)]
pub struct Dialog(String);

#[derive(Component)]
pub struct DialogOnClose(Dialog);

#[derive(Component)]
pub struct DialogNode;

fn show_dialog_on_close(
    mut commands: Commands,
    q_player: Query<(&Transform), With<Character>>,
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
        commands.spawn((
            DialogNode,
            Text::new(dialog.0.0.clone()),
            TextFont {
                font: asset_server.load("fonts/default.ttf"),
                font_size: 24.0,
                ..Default::default()
            },
        ));
    }
    // trans.translation
}

pub(super) fn register(app: &mut App) {}
