use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_lit::prelude::Lighting2dPlugin;

use crate::{
    room::{ROOM_HEIGHT, ROOM_WIDTH},
    ui::UI_HEIGHT,
};

pub mod character_controls;
pub mod enemies;
pub mod room;
pub mod sanity;
pub mod ui;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(ROOM_WIDTH, ROOM_HEIGHT + UI_HEIGHT as u32)
                    .with_scale_factor_override(1.0),
                resizable: false,
                ..default()
            }),
            ..Default::default()
        }),
        Lighting2dPlugin,
    ))
    .add_plugins(EguiPlugin::default())
    .add_plugins(WorldInspectorPlugin::default());
    character_controls::register(&mut app);
    room::register(&mut app);
    ui::register(&mut app);

    app.run();
}
    app.run();
}
