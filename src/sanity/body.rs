use bevy::prelude::*;

use crate::{
    character_controls::StatusEffects,
    light::{CheckInLight, InLight},
    menu::Playing,
    sanity::{
        DEAD_FRIEND_SANITY_BLOCKER, DEAD_SO_SANITY_AMPLIFIER, DEAD_SO_SANITY_BLOCKER, Sanity,
        SanityAmplifiers, SanityBlockers,
    },
};

#[derive(Component, Default)]
pub struct DeadBody;
#[derive(Component, Default)]
pub struct DeadSO;
#[derive(Component, Default)]
pub struct DeadFriend;

#[derive(Component)]
struct AlreadySeen;

pub fn dead_body<T: Component + Default>(
    asset_server: Res<AssetServer>,
    image: &'static str,
) -> impl Bundle {
    (
        T::default(),
        DeadBody,
        CheckInLight(32.0),
        Sprite {
            image: asset_server.load(image),
            custom_size: Some(Vec2::new(128.0, 128.0)),
            ..Default::default()
        },
    )
}

fn on_near_dead_body(
    mut commands: Commands,
    q_dead_body: Query<
        (
            Entity,
            &Transform,
            Has<InLight>,
            Has<DeadSO>,
            Has<DeadFriend>,
        ),
        (Without<AlreadySeen>, With<DeadBody>),
    >,
    mut q_player: Query<(
        &Transform,
        &mut Sanity,
        &mut SanityBlockers,
        &mut SanityAmplifiers,
        &StatusEffects,
    )>,
) {
    let Ok((player_t, mut sanity, mut blockers, mut amplifiers, status_effects)) =
        q_player.single_mut()
    else {
        return;
    };

    let Some((ent, _, _, so, friend)) = q_dead_body.iter().find(|(_, t, in_light, _, _)| {
        *in_light && t.translation.distance(player_t.translation) < 500.0
            || t.translation.distance(player_t.translation) < 20.0
    }) else {
        return;
    };

    commands.entity(ent).insert(AlreadySeen);

    if so {
        blockers.add_blocker(DEAD_SO_SANITY_BLOCKER);
        amplifiers.add_amplifier(DEAD_SO_SANITY_AMPLIFIER);
    } else if friend {
        blockers.add_blocker(DEAD_FRIEND_SANITY_BLOCKER);
    }

    sanity.clamp(&blockers);

    sanity.decrease_sanity(10.0, &amplifiers, status_effects);
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, on_near_dead_body.run_if(resource_exists::<Playing>));
}
