use bevy::{color::palettes::css, prelude::*};
use bevy_kira_audio::{Audio, AudioControl, AudioInstance};

use crate::assets::{LoadAssetsSet, load_assets};

#[derive(Resource)]
pub struct DefaultFont(Handle<Font>);

impl DefaultFont {
    pub fn get(&self) -> Handle<Font> {
        self.0.clone()
    }
}

#[derive(Resource)]
pub struct Playing;

fn show_menu(mut commands: Commands, font: Res<DefaultFont>) {
    commands
        .spawn((
            Menu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,

                ..Default::default()
            },
            BackgroundColor(css::BLACK.into()),
        ))
        .with_children(|p| {
            p.spawn((
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..Default::default()
                },
                Text::new("PLEASE USE HEADPHONES"),
                TextFont {
                    font: font.get(),
                    font_size: 64.0,
                    ..Default::default()
                },
            ));

            p.spawn((
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..Default::default()
                },
                Text::new("WASD - Move"),
                TextFont {
                    font: font.get(),
                    font_size: 40.0,
                    ..Default::default()
                },
            ));
            p.spawn((
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..Default::default()
                },
                Text::new("Arrow keys - Point Flashlight"),
                TextFont {
                    font: font.get(),
                    font_size: 40.0,
                    ..Default::default()
                },
            ));

            p.spawn((
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..Default::default()
                },
                Text::new("Space - Shine Flashlight"),
                TextFont {
                    font: font.get(),
                    font_size: 40.0,
                    ..Default::default()
                },
            ));

            p.spawn((
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..Default::default()
                },
                Text::new("Enter - Start"),
                TextFont {
                    font: font.get(),
                    font_size: 40.0,
                    ..Default::default()
                },
            ));
        });
}

#[derive(Component)]
struct Menu;

fn on_enter(
    inputs: Res<ButtonInput<KeyCode>>,
    q_menu: Query<Entity, With<Menu>>,
    mut commands: Commands,
) {
    if inputs.just_pressed(KeyCode::Enter)
        && let Ok(ent) = q_menu.single() {
            commands.insert_resource(Playing);
            commands.entity(ent).despawn();
        }
}

#[derive(Resource)]
struct BgSong(Handle<AudioInstance>);

fn play_bg(mut commands: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
    commands.insert_resource(BgSong(
        audio
            .play(asset_server.load("sounds/ambient_noise.ogg"))
            .with_volume(-50.)
            .looped()
            .handle(),
    ));
}

pub(super) fn register(app: &mut App) {
    load_assets::<Font, 1>(app, ["fonts/default.ttf"], |w, [f]| {
        w.insert_resource(DefaultFont(f));
    });

    app.add_systems(Startup, (show_menu, play_bg).after(LoadAssetsSet))
        .add_systems(Update, on_enter);
}
