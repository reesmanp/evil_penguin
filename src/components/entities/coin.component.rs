use amethyst::{
    ecs::{
        Component,
        VecStorage
    }
};

#[derive(Component, Default)]
#[storage(VecStorage)]
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
