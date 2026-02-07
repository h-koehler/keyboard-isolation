use bevy::prelude::*;

use crate::character_controls::Character;

#[derive(Component, Debug, PartialEq, PartialOrd, Reflect)]
#[require(SanityBlockers)]
pub struct Sanity(f32);

pub struct SanityBlocker {
    amount: f32,
}

impl Default for Sanity {
    fn default() -> Self {
        Self(100.0)
    }
}

impl Sanity {
    pub fn new(amt: f32) -> Self {
        Self(amt.clamp(0.0, 100.0))
    }
}

#[derive(Component, Default)]
pub struct SanityBlockers(Vec<SanityBlocker>);

impl SanityBlockers {
    pub fn maximum_sanity(&self) -> Sanity {
        Sanity::new(100.0_f32 - self.0.iter().map(|x| x.amount).sum::<f32>())
    }
}

fn add_sanity(mut commands: Commands, q_player: Query<Entity, Added<Character>>) {
    for e in q_player.iter() {
        commands.entity(e).insert(Sanity::default());
    }
}

fn clamp_sanity_to_max(mut q_sanity: Query<(&mut Sanity, &SanityBlockers)>) {
    for (mut sanity, blockers) in q_sanity.iter_mut() {
        let max = blockers.maximum_sanity();
        if *sanity > max {
            *sanity = max;
        }
        if sanity.0 < 0.0 {
            sanity.0 = 0.0;
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, (add_sanity, clamp_sanity_to_max).chain());
}
