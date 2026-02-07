use bevy::prelude::*;
use bevy_lit::prelude::PointLight2d;

#[derive(Component)]
#[require(Transform)]
pub struct Checkpoint;

#[derive(Component)]
pub struct CheckpointDone;

#[derive(Component, Default)]
pub struct CheckpointBlinking(f32);

fn on_add_checkpoint(mut commands: Commands, q_check: Query<Entity, Added<Checkpoint>>) {
    for e in q_check.iter() {
        commands.entity(e).insert((
            PointLight2d {
                inner_radius: 80.0,
                outer_radius: 120.0,
                cast_shadows: true,
                falloff: 5.0,
                intensity: 0.0,
                ..Default::default()
            },
            CheckpointBlinking(0.0),
        ));
    }
}

const BLINK_INTERVAL_SECS: f32 = 10.0;

fn blink_checkpoint(
    time: Res<Time>,
    mut q_blinking: Query<(&mut PointLight2d, &mut CheckpointBlinking)>,
) {
    let delta = time.delta_secs();
    for (mut light, mut blinking) in q_blinking.iter_mut() {
        blinking.0 += delta;
        if blinking.0 >= BLINK_INTERVAL_SECS {
            blinking.0 = BLINK_INTERVAL_SECS - blinking.0;
            light.intensity = 0.5;
        } else {
            light.intensity = 0.0;
        }
    }
}

fn spawn_checkpoint(mut commands: Commands) {
    commands.spawn((
        Checkpoint,
        CheckpointBlinking::default(),
        Transform::from_translation(Vec3::new(0.0, 400.0, 0.0)),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, spawn_checkpoint);
    app.add_systems(Update, (on_add_checkpoint, blink_checkpoint));
}
