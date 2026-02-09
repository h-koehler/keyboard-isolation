use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct AnimClip {
    pub start: u32,
    pub end: u32,
    pub fps: u32,
}


#[derive(Component)]
#[require(Sprite, AnimationState)]
pub struct AnimateSprite {
    pub default_fps: u32,
    pub anim_state: u32,
    pub clips: &'static [AnimClip], // index by anim_state
}

#[derive(Reflect, Debug, Component)]
pub struct AnimationState {
    frame: u32,
    time: f32,
    anim_state: u32,
    last_anim_state: u32,
}


impl Default for AnimationState {
    fn default() -> Self {
        Self {
            frame: 0,
            time: 0.0,
            anim_state: 0,
            last_anim_state: u32::MAX, // force initial sync
        }
    }
}

impl AnimationState {
    pub fn pause(&mut self) { self.anim_state = 0; }

    pub fn set_anim_state(&mut self, animation: u32) {
        self.anim_state = animation;
        println!("Set animation to {animation}");
    }

    fn current_clip<'a>(&self, config: &'a AnimateSprite) -> Option<AnimClip> {
        let idx = self.anim_state as usize;
        config.clips.get(idx).copied()
    }

    fn sync_if_changed(&mut self, config: &AnimateSprite) {
        if self.anim_state != self.last_anim_state {
            // animation changed -> restart the new clip
            self.time = 0.0;

            if let Some(clip) = self.current_clip(config) {
                self.frame = clip.start;
            } else {
                self.frame = 0;
            }

            self.last_anim_state = self.anim_state;
        }
    }

    fn tick(&mut self, delta: f32, config: &AnimateSprite) {
        // If you still want AnimateSprite.anim_state to “drive” it, copy it ONCE HERE:
        //self.anim_state = config.anim_state;

        self.sync_if_changed(config);

        if self.anim_state == 999 {
            return; // paused
        }

        let Some(clip) = self.current_clip(config) else {
            return;
        };

        self.time += delta;
        let fps = if clip.fps == 0 { config.default_fps } else { clip.fps };
        let time_per_frame = 1.0 / fps as f32;

        while self.time >= time_per_frame {
            self.time -= time_per_frame;

            if self.frame < clip.start || self.frame > clip.end {
                self.frame = clip.start;
            } else if self.frame == clip.end {
                self.frame = clip.start; // loop
            } else {
                self.frame += 1;
            }
        }
    }
}

fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&AnimateSprite, &mut AnimationState, &mut Sprite)>,
) {
    for (config, mut state, mut sprite) in &mut query {
        let Some(atlas) = &mut sprite.texture_atlas else { continue; };

        state.tick(time.delta_secs(), config);

        let desired = state.frame as usize;
        if atlas.index != desired {
            atlas.index = desired;
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, execute_animations);
}
