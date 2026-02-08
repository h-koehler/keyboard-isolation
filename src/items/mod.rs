use bevy::{platform::collections::HashSet, prelude::*};

use crate::character_controls::Character;

pub const PICKUP_DIST: f32 = 50.0;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Reflect)]
pub enum Item {
    Antenna,
    Chip,
    Plate,
    Flower,
    Flashlight,
}

#[derive(Component)]
pub struct CollectableItem(Item);

#[derive(Component, Reflect)]
pub struct CollectedItems(pub HashSet<Item>);

impl CollectedItems {
    pub fn collect_item(&mut self, item: Item) {
        self.0.insert(item);
    }

    pub fn get_item(&mut self, item: Item) {
        self.0.get(&item);
    }

    pub fn iter(&self) -> impl Iterator<Item = Item> {
        self.0.iter().copied()
    }
}

fn item(asset_server: &AssetServer, item: Item, item_name: &str, asset_name: &str) -> impl Bundle {
    (
        Name::new(item_name.to_string()),
        CollectableItem(item),
        Sprite {
            image: asset_server.load(asset_name.to_string()),
            custom_size: Some(Vec2::splat(45.0)),
            ..Default::default()
        },
    )
}

fn pickup_item(
    mut commands: Commands,
    mut q_item: Query<(&mut Transform, &mut CollectableItem, Entity), With<CollectableItem>>,
    mut q_player: Query<&Transform, (With<Character>, Without<CollectableItem>)>,
    mut q_collected_items: Query<&mut CollectedItems>,
) {
    let player_transform = q_player.single_mut().expect("No Player Object");
    let player_translation = player_transform.translation.truncate();

    for (item_transform, item, item_ent) in q_item.iter_mut() {
        let item_translation = item_transform.translation.truncate();
        let difference = player_translation - item_translation;

        if difference.length() <= PICKUP_DIST {
            let mut collected_items = q_collected_items
                .single_mut()
                .expect("No Collected Item Object");
            collected_items.collect_item(item.0);
            commands.entity(item_ent).despawn();
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        item(&asset_server, Item::Antenna, "Antenna", "antenna.png"),
        Transform::from_translation(Vec3::new(-100.0, 100.0, 3.0)),
    ));

    commands.spawn((
        item(&asset_server, Item::Plate, "Plate", "plate.png"),
        Transform::from_translation(Vec3::new(340.0, -450.0, 3.0)),
    ));

    commands.spawn((
        item(&asset_server, Item::Chip, "Chip", "chip.png"),
        Transform::from_translation(Vec3::new(1325.0, 505.0, 3.0)),
    ));

    commands.spawn((
        item(&asset_server, Item::Flower, "Flower", "flower.png"),
        Transform::from_translation(Vec3::new(-650.0, -890.0, 3.0)),
    ));

    commands.spawn((
        item(
            &asset_server,
            Item::Flashlight,
            "Flashlight",
            "flashlight.png",
        ),
        Transform::from_translation(Vec3::new(200.0, 1500.0, 3.0)),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, pickup_item);
}
