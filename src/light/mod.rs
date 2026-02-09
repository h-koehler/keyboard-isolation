use bevy::prelude::*;
use bevy_lit::prelude::{PointLight2d, SpotLight2d};
use rand::Rng;

use crate::{
    character_controls::SpawnEnemies,
    enemies::{Enemy, StalkerAsset, alien, stalker, teleporting_alien},
    sanity::Sanity,
};

use std::f32::consts::TAU;

#[derive(Component, Reflect)]
/// The pub f32 is the radius of the things you're checking
pub struct CheckInLight(pub f32);

#[derive(Component, Reflect)]
pub struct InLight;

#[derive(Component)]
pub struct IgnoreInLightCheckLight;

fn check_in_light(
    q_check: Query<(Entity, &Transform, &CheckInLight)>,
    q_light: Query<(Entity, &GlobalTransform, &PointLight2d), Without<IgnoreInLightCheckLight>>,
    q_spotlight: Query<
        (&GlobalTransform, &SpotLight2d, Option<&ChildOf>),
        Without<IgnoreInLightCheckLight>,
    >,
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
        }) || q_spotlight.iter().any(|(g_trans, light, child_of)| {
            if light.intensity == 0.0 {
                return false;
            }

            if child_of.is_some_and(|x| x.parent() == ent) {
                return true;
            }

            let moved_trans = g_trans.translation() - check_trans.translation;

            if moved_trans.length() > 1000.0 {
                return false;
            }

            let dotted = moved_trans.normalize_or_zero().dot(g_trans.left().into());

            dotted > (1.0 - light.outer_angle / 90.0)
        }) {
            commands.entity(ent).insert(InLight);
        } else {
            commands.entity(ent).remove::<InLight>();
        }
    }
}

fn spawn_enemy(
    mut q_player: Query<(&Transform, &Sanity, &mut SpawnEnemies)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let (player_transform, player_sanity, mut player_spawn_enemies) =
        q_player.single_mut().expect("No Player Object");
    player_spawn_enemies.stopwatch.tick(time.delta());
    let time_to_spawn_enemies = match player_sanity.0 {
        0.0..25.0 => 5.0,
        25.0..75.0 => 10.0,
        _ => 20.0,
    };
    if player_spawn_enemies.stopwatch.elapsed_secs() > time_to_spawn_enemies {
        let mut rng = rand::rng();
        let random_angle = rng.random_range(0.0..TAU);
        let dir = Vec2::from_angle(random_angle);
        commands.spawn((
            Enemy,
            CheckInLight(1.0),
            Transform::from_translation(
                (player_transform.translation.truncate() + dir * 300.0).extend(0.0),
            ),
        ));
        player_spawn_enemies.stopwatch.reset();
    }
}

fn despawn_enemies_spawned_in_light(
    q_enemies: Query<Entity, (With<Enemy>, Without<Sprite>, With<InLight>)>,
    mut commands: Commands,
) {
    for enemy in q_enemies.iter() {
        commands.entity(enemy).despawn();
    }
}

fn finish_enemies_spawned_in_darkness(
    q_enemies: Query<Entity, (With<Enemy>, Without<Sprite>, Without<InLight>)>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    stalker_asset: Res<StalkerAsset>,
    mut commands: Commands,
) {
    let mut rng = rand::rng();
    for enemy in q_enemies.iter() {
        let random: f32 = rng.random();
        if random < 0.34 {
            commands
                .entity(enemy)
                .insert(alien(&asset_server, &mut meshes));
        } else if random < 0.67 {
            commands
                .entity(enemy)
                .insert(teleporting_alien(&asset_server, &mut meshes));
        } else {
            commands
                .entity(enemy)
                .insert(stalker(&stalker_asset, &asset_server, &mut meshes));
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(
        First,
        (
            spawn_enemy,
            check_in_light,
            despawn_enemies_spawned_in_light,
            finish_enemies_spawned_in_darkness,
        )
            .chain(),
    );
}
