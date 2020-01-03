use amethyst::{
    ecs::{Component, DenseVecStorage},
    core::math::Vector3,
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct MovementComponent {
    pub velocity: Vector3<f32>,
    pub max_speed: f32,
    pub friction: f32
}

impl MovementComponent {
    pub fn new(velocity: Vector3<f32>, max_speed: f32, friction: f32) -> Self {
        Self {
            velocity,
            max_speed,
            friction
        }
    }
}
