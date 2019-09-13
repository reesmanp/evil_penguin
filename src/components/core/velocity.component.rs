use amethyst::{
    ecs::{Component, DenseVecStorage}
};

pub struct VelocityComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub acceleration_magnitude: f32,
    pub max_speed: f32,
    pub coast_magnitude: f32
}

impl VelocityComponent {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        acceleration_magnitude: f32,
        max_speed: f32,
        coast_magnitude: f32
    ) -> Self {
        Self {
            x,
            y,
            z,
            acceleration_magnitude,
            max_speed,
            coast_magnitude
        }
    }
}

impl Component for VelocityComponent {
    type Storage = DenseVecStorage<Self>;
}
