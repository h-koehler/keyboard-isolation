use bevy::prelude::*;

#[derive(Component)]
#[require(Sprite, AnimationTracker)]
pub struct AnimateSprite {
    pub n_frames: u32,
    pub fps: u32,
}

#[derive(Reflect, Debug, Component, Default)]
struct AnimationTracker {
    frame: u32,
    time: f32,
}

impl AnimationTracker {
    fn tick(&mut self, delta: f32, config: &AnimateSprite) {
        self.time += delta;

        let time_per_frame = 1.0 / config.fps as f32;
        while self.time >= time_per_frame {
            self.time -= time_per_frame;
            self.frame += 1;
        }
        self.frame %= config.n_frames;
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimateSprite`).
fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&AnimateSprite, &mut AnimationTracker, &mut Sprite)>,
) {
    for (config, mut state, mut sprite) in &mut query {
        // We track how long the current sprite has been displayed for
        state.tick(time.delta_secs(), config);

        let Some(atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        if atlas.index != state.frame as usize {
            atlas.index = state.frame as usize;
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, execute_animations);
}
