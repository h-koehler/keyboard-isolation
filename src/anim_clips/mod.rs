use crate::{
    animation::AnimClip
};

pub const fire_clips: &[AnimClip] = &[
    AnimClip { start: 0,  end: 6,  fps: 10} // FIRE
];

pub const PLAYER_CLIPS: &[AnimClip] = &[
    AnimClip { start: 0,  end: 0,  fps: 0 },//IDLE
    AnimClip { start: 0,  end: 5,  fps: 10 }, //WALK
    // AnimClip { start: 0,  end: 3,  fps: 8 },
      

    // AnimClip { start: 4,  end: 11, fps: 12 },
    // AnimClip { start: 12, end: 19, fps: 16 },
];
