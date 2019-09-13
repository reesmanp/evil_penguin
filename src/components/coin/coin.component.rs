use amethyst::{
    ecs::{
        Component,
        VecStorage
    }
};

use crate::constants::COIN_TIME_PER_FRAME;

pub struct CoinComponent {
    pub frame: usize,
    pub frames: usize,
    pub time_per_frame: f32,
    pub time_elapsed: f32
}

impl CoinComponent {
    pub fn new(
        frame: usize,
        frames: usize,
        time_per_frame: f32,
        time_elapsed: f32
    ) -> Self
    {
        Self {
            frame,
            frames,
            time_per_frame,
            time_elapsed
        }
    }

    pub fn elapse_time(&mut self, time: f32) {
        self.time_elapsed += time;
    }

    pub fn update_frame(&mut self) {
        self.frame = (self.time_elapsed / self.time_per_frame) as usize % self.frames;
    }
}

impl Component for CoinComponent {
    type Storage = VecStorage<Self>;
}

impl Default for CoinComponent {
    fn default() -> Self {
        Self::new(0, 0, COIN_TIME_PER_FRAME, 0.0)
    }
}
