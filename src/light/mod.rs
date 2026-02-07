use bevy::prelude::*;
use bevy_lit::prelude::{PointLight2d, SpotLight2d};

#[derive(Component, Reflect)]
/// The pub f32 is the radius of the things you're checking
pub struct CheckInLight(pub f32);

#[derive(Component, Reflect)]
pub struct InLight;

#[derive(Component)]
pub struct IgnoreInLightCheckLight;

fn check_in_light(
    q_check: Query<(Entity, &Transform, &CheckInLight), Without<IgnoreInLightCheckLight>>,
    q_light: Query<(Entity, &GlobalTransform, &PointLight2d), Without<IgnoreInLightCheckLight>>,
    q_spotlight: Query<(&GlobalTransform, &SpotLight2d)>,
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
        }) || q_spotlight.iter().any(|(g_trans, light)| {
            let moved_trans = check_trans.translation;
            let dotted = moved_trans.normalize_or_zero().dot(g_trans.left().into());

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
