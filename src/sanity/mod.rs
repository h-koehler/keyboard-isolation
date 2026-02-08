use std::time::Duration;

use bevy::{color::palettes::css, prelude::*};

use crate::character_controls::Character;

pub mod body;
pub mod player_sanity;

#[derive(Component, Debug, PartialEq, PartialOrd, Reflect)]
#[require(SanityBlockers, SanityAmplifiers)]
pub struct Sanity(f32);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Reflect)]
pub struct SanityBlocker {
    pub amount: f32,
    pub duration: Duration,
}

pub const HURT_SANITY_BLOCKER: SanityBlocker = SanityBlocker {
    amount: 10.0,
    duration: Duration::from_secs(120),
};
pub const DEAD_FRIEND_SANITY_BLOCKER: SanityBlocker = SanityBlocker {
    amount: 10.0,
    duration: Duration::from_secs(120),
};
pub const DEAD_SO_SANITY_BLOCKER: SanityBlocker = SanityBlocker {
    amount: 20.0,
    duration: Duration::from_secs(120),
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Reflect)]
pub struct SanityAmplifier {
    pub amount: f32,
    pub duration: Duration,
}

pub const DRUG_SANITY_AMPLIFIER: SanityAmplifier = SanityAmplifier {
    amount: 4.0,
    duration: Duration::from_secs(120),
};
pub const DEAD_SO_SANITY_AMPLIFIER: SanityAmplifier = SanityAmplifier {
    amount: 2.0,
    duration: Duration::from_secs(120),
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Reflect)]
pub struct SanityBoost {
    pub amount: f32,
    pub duration: Duration,
}

#[derive(Clone, Component, Debug, PartialEq, PartialOrd, Reflect, Default)]
pub struct SanityAmplifiers(Vec<SanityAmplifier>);

impl SanityAmplifiers {
    pub fn add_amplifier(&mut self, amplifier: SanityAmplifier) {
        self.0.push(amplifier);
    }

    pub fn multiplier(&self) -> f32 {
        1.0 + self.0.iter().map(|x| x.amount).sum::<f32>()
    }
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

    pub fn clamp(&mut self, blockers: &SanityBlockers) {
        self.0 = self.0.clamp(0.0, blockers.maximum_sanity().0);
    }

    pub fn decrease_sanity(&mut self, amount_to_remove: f32, amplifiers: &SanityAmplifiers) {
        *self = Self::new(self.0 - amount_to_remove * amplifiers.multiplier());
    }

    pub fn increase_sanity(
        &mut self,
        amount_to_increase: f32,
        amplifiers: &SanityAmplifiers,
        blockers: &SanityBlockers,
    ) {
        *self = Self::new(
            (self.0 + amount_to_increase * amplifiers.multiplier())
                .min(blockers.maximum_sanity().0),
        );
    }
}

#[derive(Component, Default, Reflect)]
pub struct SanityBlockers(Vec<(SanityBlocker, f32)>);

impl SanityBlockers {
    pub fn tick_blockers(&mut self, delta: f32) {
        for blocker in self.0.iter_mut() {
            blocker.1 += delta;
        }

        self.0.retain(|x| x.0.duration.as_secs_f32() < x.1);
    }

    pub fn add_blocker(&mut self, blocker: SanityBlocker) {
        self.0.push((blocker, 0.0));
    }

    pub fn maximum_sanity(&self) -> Sanity {
        Sanity::new(
            100.0_f32
                - self
                    .0
                    .iter()
                    .map(|(x, time_had)| {
                        x.amount
                            * (1.0
                                - ((*time_had - x.duration.as_secs_f32() / 2.0).max(0.0)
                                    / (x.duration.as_secs_f32() / 2.0))
                                    .clamp(0.0, 1.0))
                    })
                    .sum::<f32>(),
        )
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

#[derive(Component, Reflect)]
struct SanityBar(f32);

fn add_sanity_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                top: Val::Px(50.0),
                left: Val::Px(50.0),
                width: Val::Px(300.0),
                height: Val::Px(50.0),
                ..Default::default()
            },
            Name::new("Sanity Bar"),
        ))
        .with_children(|p| {
            p.spawn((
                ImageNode {
                    image: asset_server.load("sanity_circle.png"),
                    color: css::PURPLE.into(),
                    ..Default::default()
                },
                SanityColorCircle,
                Node {
                    width: Val::Px(128.0),
                    height: Val::Px(128.0),
                    ..Default::default()
                },
                ZIndex(10),
            ))
            .with_children(|p| {
                p.spawn((
                    ImageNode {
                        image: asset_server.load("sanity_brain.png"),
                        ..Default::default()
                    },
                    Node {
                        width: Val::Px(128.0),
                        height: Val::Px(128.0),
                        ..Default::default()
                    },
                ));
            });

            p.spawn((Node {
                top: Val::Px(40.0),
                left: Val::Px(-32.8),
                width: Val::Px(600.0),
                height: Val::Px(87.0),
                margin: UiRect::AUTO,
                ..Default::default()
            },))
                .with_children(|p| {
                    p.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Px(600.0),
                            height: Val::Px(87.0),
                            ..Default::default()
                        },
                        SanityBar(600.0),
                        BackgroundColor(css::MEDIUM_PURPLE.into()),
                    ));
                    p.spawn((
                        ZIndex(1),
                        ImageNode {
                            image: asset_server.load("sanity_bar.png"),
                            ..Default::default()
                        },
                        Node {
                            top: Val::Px(-40.0),
                            height: Val::Px(160.0),
                            // left: Val::Px(-10.0),
                            ..Default::default()
                        },
                    ));
                });
        });
}

#[derive(Component)]
struct SanityColorCircle;

fn update_sanity_bar(
    q_sanity: Query<&Sanity>,
    mut q_sanity_bar: Query<(&mut BackgroundColor, &mut Node, &SanityBar)>,
    mut q_color: Query<&mut ImageNode, With<SanityColorCircle>>,
) {
    let Ok(sanity) = q_sanity.single() else {
        return;
    };

    let Ok((mut bg, mut node, sanity_bar)) = q_sanity_bar.single_mut() else {
        return;
    };

    node.width = Val::Px(sanity.0 / 100.0 * sanity_bar.0);

    let color = if sanity.0 >= 75.0 {
        css::WHITE
    } else if sanity.0 >= 25.0 {
        css::PURPLE
    } else {
        css::RED
    }
    .into();

    bg.0 = color;
    if let Ok(mut n) = q_color.single_mut() {
        n.color = color;
    }
}

fn tick_sanity(mut q_sanity: Query<&mut SanityBlockers>, time: Res<Time>) {
    let delta = time.delta_secs();
    for mut blocker in q_sanity.iter_mut() {
        blocker.tick_blockers(delta);
    }
}

pub(super) fn register(app: &mut App) {
    player_sanity::register(app);
    body::register(app);

    app.add_systems(
        Update,
        (
            add_sanity,
            tick_sanity,
            clamp_sanity_to_max,
            update_sanity_bar,
        )
            .chain(),
    )
    .add_systems(Startup, add_sanity_bar);
}
