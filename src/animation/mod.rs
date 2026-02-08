use bevy::prelude::*;

#[derive(Component)]
#[require(Sprite, AnimationState)]
pub struct AnimateSprite {
    pub n_frames: u32,
    pub fps: u32,
}

#[derive(Reflect, Debug, Component, Default)]
pub struct AnimationState {
    frame: u32,
    time: f32,
    frozen: bool,
}

impl AnimationState {
    pub fn pause(&mut self) {
        self.frozen = true;
    }

    pub fn resume(&mut self) {
        self.frozen = false;
    }

    pub fn set_frame(&mut self, frame: u32) {
        self.frame = frame;
    }

    fn tick(&mut self, delta: f32, config: &AnimateSprite) {
        if self.frozen {
            return;
        };

        self.time += delta;

        let time_per_frame = 1.0 / config.fps as f32;
        while self.time >= time_per_frame {
            self.time -= time_per_frame;
            self.frame += 1;
        }
        self.frame %= config.n_frames;
    }
}

fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&AnimateSprite, &mut AnimationState, &mut Sprite)>,
) {
    for (config, mut state, mut sprite) in &mut query {
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
