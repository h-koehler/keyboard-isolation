use bevy::prelude::*;
use bevy_lit::prelude::{PointLight2d, SpotLight2d};

#[derive(Component, Reflect)]
pub struct CheckInLight(pub f32);

#[derive(Component, Reflect)]
pub struct InLight;

#[derive(Component)]
pub struct IgnoreInLightCheckLight;

fn check_in_light(
    q_check: Query<(Entity, &Transform, &CheckInLight), Without<IgnoreInLightCheckLight>>,
    q_light: Query<(Entity, &GlobalTransform, &PointLight2d), Without<IgnoreInLightCheckLight>>,
    q_spotlight: Query<(&ChildOf, &SpotLight2d)>,
    q_g_trans: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    for (ent, check_trans, check) in q_check.iter() {
        if q_light.iter().any(|(light_ent, trans, light)| {
            if light_ent == ent {
                return false;
            }
            light.intensity != 0.0
                && (trans.translation() - check_trans.translation).length()
                    < (light.outer_radius + check.0)
        }) || q_spotlight.iter().any(|(child_of, light)| {
            // if child_of.parent() == ent {
            //     return false;
            // }
            let Ok(parent_trans) = q_g_trans.get(child_of.parent()) else {
                return false;
            };

            let dotted = (parent_trans.rotation().inverse() * check_trans.translation)
                .normalize_or_zero()
                .dot(parent_trans.translation().normalize_or_zero());

            println!("{dotted} vs {}", light.outer_angle);
            dotted > (1.0 - light.outer_angle / 90.0)
        }) {
            commands.entity(ent).insert(InLight);
        } else {
            commands.entity(ent).remove::<InLight>();
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(First, check_in_light);
}
