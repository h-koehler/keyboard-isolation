use bevy::{color::palettes::css, prelude::*};
use bevy_lit::prelude::SpotLight2d;

#[derive(Component)]
pub struct FlashlightActive;

#[derive(Component, Reflect)]
pub struct Flashlight {
    /// Seconds of battery life remaining
    pub battery: f32,
    pub max_charge: f32,
}

pub fn update_flashlight(
    mut q_flashlight: Query<(Entity, &mut Flashlight, &mut SpotLight2d)>,
    inputs: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let enabled = inputs.pressed(KeyCode::Space);

    for (ent, mut flashlight, mut spotlight) in q_flashlight.iter_mut() {
        if enabled && flashlight.battery > 0.0 {
            flashlight.battery -= time.delta_secs();
            spotlight.intensity = 1.0;
            flashlight.battery = flashlight.battery.max(0.0);
            commands.entity(ent).insert(FlashlightActive);
        } else {
            spotlight.intensity = 0.0;
            commands.entity(ent).remove::<FlashlightActive>();
        }
    }
}

#[derive(Component)]
struct BatteryAmount(f32);

fn add_flashlight(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                top: Val::Px(100.0),
                right: Val::Px(100.0),
                margin: UiRect::left(Val::Auto),
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                ..Default::default()
            },
            Name::new("Battery UI"),
        ))
        .with_children(|p| {
            p.spawn((
                ImageNode {
                    image: asset_server.load("battery.png"),
                    ..Default::default()
                },
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
            ))
            .with_children(|p| {
                p.spawn((
                    Node {
                        width: Val::Percent(64.0),
                        height: Val::Percent(85.0),
                        margin: UiRect::AUTO,
                        ..Default::default()
                    },
                    BatteryAmount(85.0),
                    BackgroundColor(css::LIGHT_YELLOW.into()),
                ));

                p.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        margin: UiRect::AUTO,
                        width: Val::Px(64.0),
                        height: Val::Px(64.0),
                        ..Default::default()
                    },
                    ImageNode {
                        image: asset_server.load("battery_electric.png"),
                        ..Default::default()
                    },
                ));
            });
        });
}

fn update_flashlight_battery(
    mut q_bat_node: Query<(&mut Node, &BatteryAmount)>,
    q_flashlight: Query<&Flashlight>,
) {
    let Ok(flashlight) = q_flashlight.single() else {
        return;
    };
    let Ok((mut n, ui_amt)) = q_bat_node.single_mut() else {
        return;
    };
    let percent = (flashlight.battery / flashlight.max_charge) * ui_amt.0;
    let top = (ui_amt.0 - percent) / 2.0;
    n.height = Val::Percent(percent);
    n.top = Val::Percent(top);
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, add_flashlight);
    app.add_systems(Update, (update_flashlight, update_flashlight_battery));
}
