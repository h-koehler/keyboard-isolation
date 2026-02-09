use bevy::prelude::*;

use crate::{
    character_controls::{StatusEffect, StatusEffects},
    light::InLight,
    sanity::{Sanity, SanityAmplifiers, SanityBlockers},
};

fn decrease_sanity_in_dark(
    mut q_sanity: Query<(
        &mut Sanity,
        Has<InLight>,
        &SanityAmplifiers,
        &SanityBlockers,
        &StatusEffects,
    )>,
    time: Res<Time>,
) {
    for (mut sanity, in_light, amplifiers, blockers, status_effects) in q_sanity.iter_mut() {
        if in_light {
            sanity.increase_sanity(
                1.0 * time.delta_secs(),
                amplifiers,
                blockers,
                status_effects,
            );
        } else {
            sanity.decrease_sanity(1.0 * time.delta_secs(), amplifiers, status_effects);
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, (decrease_sanity_in_dark).chain());
}
