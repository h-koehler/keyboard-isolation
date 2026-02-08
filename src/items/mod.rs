use bevy::{platform::collections::HashSet, prelude::*};

pub enum Item {
    BeaconPart(BeaconPart),
    Flower,
    Flashlight,
}

pub enum BeaconPart {
    Antenna,
    Plate,
    Chip,
}

#[derive(Component)]
pub struct LostBeaconPart(BeaconPart);

#[derive(Component)]
pub struct CollectableItem(Item);

fn item(asset_server: &AssetServer, item: Item, item_name: &str) -> impl Bundle {
    let x = 1;

    (
        Name::new(item_name),
        Item,
        Sprite {
            image: asset_server.load("dog.png"),
            custom_size: Some(Vec2::splat(45.0)),
            ..Default::default()
        },
    )
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        beacon_part(&asset_server, BeaconPart::Antenna, "Antenna"),
        Transform::from_translation(Vec3::new(-650.0, 770.0, 3.0)),
    ));

    commands.spawn((
        beacon_part(&asset_server, BeaconPart::Plate),
        Transform::from_translation(Vec3::new(-650.0, 770.0, 3.0)),
    ));

    commands.spawn((
        beacon_part(&asset_server, BeaconPart::Chip),
        Transform::from_translation(Vec3::new(-650.0, 770.0, 3.0)),
    ));

    commands.spawn((
        beacon_part(&asset_server, BeaconPart::Antenna),
        Transform::from_translation(Vec3::new(-650.0, 770.0, 3.0)),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, setup);
}
