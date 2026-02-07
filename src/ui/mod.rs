use bevy::{color::palettes::css, prelude::*};

pub const UI_HEIGHT: f32 = 200.0;

fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                bottom: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(UI_HEIGHT),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(20.0)),
                ..Default::default()
            },
            BackgroundColor(css::WHITE.into()),
        ))
        .with_children(|p| {
            p.spawn((
                Node {
                    margin: UiRect::vertical(Val::Auto),
                    width: Val::Px(128.0),
                    height: Val::Px(128.0),
                    ..Default::default()
                },
                ImageNode::new(asset_server.load("X.png")),
            ));

            p.spawn((
                Node {
                    margin: UiRect::vertical(Val::Auto),
                    width: Val::Px(128.0),
                    height: Val::Px(128.0),
                    ..Default::default()
                },
                ImageNode::new(asset_server.load("red.png")),
            ));
        });
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, create_ui);
}
