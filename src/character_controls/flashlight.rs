use bevy::prelude::*;
use bevy_lit::prelude::SpotLight2d;

#[derive(Component)]
pub struct FlashlightActive;

#[derive(Component)]
pub struct Flashlight {
    /// Seconds of battery life remaining
    pub battery: f32,
    pub max_charge: f32,
}

fn update_flashlight(
    mut q_flashlight: Query<(Entity, &mut Flashlight, &mut SpotLight2d)>,
    inputs: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let enabled = inputs.pressed(KeyCode::Space);

    for (ent, mut flashlight, mut spotlight) in q_flashlight.iter_mut() {
        if enabled {
            flashlight.max_charge -= time.delta_secs();
            spotlight.intensity = 1.0;
            commands.entity(ent).insert(FlashlightActive);
        } else {
            spotlight.intensity = 0.0;
            commands.entity(ent).remove::<FlashlightActive>();
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, update_flashlight);
}
