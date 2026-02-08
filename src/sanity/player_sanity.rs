use bevy::prelude::*;

use crate::{
    character_controls::{Character, StatusEffect, StatusEffects},
    light::InLight,
    sanity::{Sanity, SanityAmplifiers, SanityBlockers},
};

fn decrease_sanity_in_dark(
    mut q_sanity: Query<(
        &mut Sanity,
        Has<InLight>,
        &SanityAmplifiers,
        &SanityBlockers,
    )>,
    time: Res<Time>,
) {
    for (mut sanity, in_light, amplifiers, blockers) in q_sanity.iter_mut() {
        if in_light {
            sanity.increase_sanity(1.0 * time.delta_secs(), amplifiers, blockers);
        } else {
            sanity.decrease_sanity(1.0 * time.delta_secs(), amplifiers);
        }
    }
}

fn update_insane_status(mut q_player: Query<(&Sanity, &mut StatusEffects)>) {
    let (sanity, mut status_effects) = q_player.single_mut().expect("No Player Object");
    if sanity.0 < 25.0 {
        status_effects.add_effect(StatusEffect::Insane);
    } else {
        status_effects.remove_effect(StatusEffect::Insane);
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(
        Update,
        (decrease_sanity_in_dark, update_insane_status).chain(),
    );
}
