use bevy::prelude::*;

use crate::{
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

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, decrease_sanity_in_dark);
}
