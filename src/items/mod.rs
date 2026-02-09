use bevy::{platform::collections::HashSet, prelude::*};
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

use crate::{
    character_controls::{Character, StatusEffect, StatusEffects},
    menu::Playing,
    sanity::{Sanity, SanityAmplifiers, SanityBlockers},
    win::SoundHandle,
};

pub const PICKUP_DIST: f32 = 50.0;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Reflect)]
pub enum Item {
    Antenna,
    Chip,
    Plate,
    Flower,
    MedPack,
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

#[derive(Resource)]
pub struct PickupSound(Handle<AudioSource>);

fn load_pickup_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PickupSound(asset_server.load("sounds/item_pickup.ogg")));
}

fn pickup_item(
    mut commands: Commands,
    mut q_item: Query<(&mut Transform, &mut CollectableItem, Entity), With<CollectableItem>>,
    mut q_player: Query<
        (
            &Transform,
            &mut Character,
            &mut Sanity,
            &SanityAmplifiers,
            &SanityBlockers,
            &mut StatusEffects,
        ),
        (With<Character>, Without<CollectableItem>),
    >,
    mut q_collected_items: Query<&mut CollectedItems>,
    pickup_sound: Res<PickupSound>,
    audio: Res<Audio>,
) {
    let (
        player_transform,
        mut player_character,
        mut sanity,
        sanity_amplifiers,
        sanity_blockers,
        mut status_effects,
    ) = q_player.single_mut().expect("No Player Object");
    let player_translation = player_transform.translation.truncate();

    for (item_transform, item, item_ent) in q_item.iter_mut() {
        let item_translation = item_transform.translation.truncate();
        let difference = player_translation - item_translation;

        if difference.length() <= PICKUP_DIST {
            let mut collected_items = q_collected_items
                .single_mut()
                .expect("No Collected Item Object");
            collected_items.collect_item(item.0);

            match item.0 {
                Item::Flower => sanity.increase_sanity(
                    50.0,
                    sanity_amplifiers,
                    sanity_blockers,
                    &status_effects,
                ),
                Item::MedPack => {
                    player_character.heal();
                    status_effects.remove_effect(StatusEffect::Slowed);
                }
                _ => {}
            }

            commands.entity(item_ent).despawn();

            commands.spawn((SoundHandle(
                audio
                    .play(pickup_sound.0.clone())
                    .with_volume(-10.0)
                    .handle(),
            ),));
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        item(&asset_server, Item::Antenna, "Antenna", "antenna.png"),
        Transform::from_translation(Vec3::new(-6000.0, 2550.0, 3.0)),
    ));

    commands.spawn((
        item(&asset_server, Item::Plate, "Plate", "plate.png"),
        Transform::from_translation(Vec3::new(6000.0, 2550.0, 3.0)),
    ));

    commands.spawn((
        item(&asset_server, Item::Chip, "Chip", "chip.png"),
        Transform::from_translation(Vec3::new(3500.0, -2850.0, 3.0)),
    ));

    commands.spawn((
        item(&asset_server, Item::Flower, "Flower", "flower.png"),
        Transform::from_translation(Vec3::new(3660.0, -2920.0, 3.0)),
    ));

    commands.spawn((
        item(&asset_server, Item::MedPack, "Med Pack", "med_pack.png"),
        Transform::from_translation(Vec3::new(6000.0, 2500.0, 3.0)),
    ));

    commands.spawn((
        item(
            &asset_server,
            Item::Flashlight,
            "Flashlight",
            "flashlight.png",
        ),
        Transform::from_translation(Vec3::new(-20.0, 360.0, 3.0)),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, (setup, load_pickup_sound));
    app.add_systems(Update, pickup_item.run_if(resource_exists::<Playing>));
}
