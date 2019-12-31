use amethyst::{
    ecs::{Component, DenseVecStorage},
    core::math::Vector3,
    renderer::sprite::Sprite
};
use std::cmp::Ordering;
use crate::util::constants::{DEFAULT_ARENA_HEIGHT, DEFAULT_ARENA_WIDTH};

pub struct MovementComponent {
    pub position: Vector3<f32>,
    old_position: Vector3<f32>,
    velocity: Vector3<f32>,
    max_speed: f32,
    friction: f32
}

impl MovementComponent {
    pub fn new(position: Vector3<f32>, velocity: Vector3<f32>, max_speed: f32, friction: f32) -> Self {
        Self {
            old_position: position.clone(),
            position,
            velocity,
            max_speed,
            friction
        }
    }

    pub fn accelerate(&mut self, acceleration: Vector3<f32>, seconds: f32, sprite: &Sprite, scale: &Vector3<f32>) {
        let x_accel = self.get_actual_acceleration_component(acceleration.x, self.velocity.x);
        let y_accel = self.get_actual_acceleration_component(acceleration.y, self.velocity.y);
        let acceleration_vec = Vector3::new(x_accel, y_accel, acceleration.z);
        let mut new_velocity = self.velocity + acceleration_vec;

        // If no x acceleration
        if acceleration.x == 0.0 {
            // Do not pass 0 during slowdown
            if self.velocity.x > 0.0 && new_velocity.x < 0.0 {
                new_velocity.x = 0.0;
            } else if self.velocity.x < 0.0 && new_velocity.x > 0.0 {
                new_velocity.x = 0.0;
            }
        }

        // If no y acceleration
        if acceleration.y == 0.0 {
            // Do not pass 0 during slowdown
            if self.velocity.y > 0.0 && new_velocity.y < 0.0 {
                new_velocity.y = 0.0;
            } else if self.velocity.y < 0.0 && new_velocity.y > 0.0 {
                new_velocity.y = 0.0;
            }
        }

        // Crash Scenario
        if self.position.x < 0.0 {
            self.position.x = 0.01;
            new_velocity.x = 0.0;
        } else if self.position.x + sprite.width * scale.x > DEFAULT_ARENA_WIDTH {
            self.position.x = DEFAULT_ARENA_WIDTH - sprite.width * scale.x;
            new_velocity.x = 0.0;
        }

        // Crash Scenario
        if self.position.y < 0.0 {
            self.position.y = 0.0;
            new_velocity.y = 0.0;
        } else if self.position.y + sprite.height * scale.y > DEFAULT_ARENA_HEIGHT {
            self.position.y = DEFAULT_ARENA_HEIGHT - sprite.height * scale.y;
            new_velocity.y = 0.0;
        }

        let current_speed = new_velocity.magnitude();
        if current_speed < self.max_speed {
            self.velocity = new_velocity;
        }

        self.old_position = self.position;
        self.position += self.velocity * seconds;
    }

    fn get_actual_acceleration_component(&self, acceleration_component: f32, velocity_component: f32) -> f32 {
        // If the acceleration component is zero, start slowing down
        if acceleration_component == 0.0 {
            if velocity_component.eq(&0.0) {
                0.0
            } else {
                self.friction * (-velocity_component / velocity_component.abs())
            }
        } else {
            5.0 * (acceleration_component / acceleration_component.abs())
        }
    }

    pub fn get_delta_x(&self) -> f32 {
        self.position.x - self.old_position.x
    }

    pub fn get_delta_y(&self) -> f32 {
        self.position.y - self.old_position.y
    }

    pub fn get_delta_z(&self) -> f32 {
        self.position.z - self.old_position.z
    }

    pub fn get_velocity_vector(&self) -> Vector3<f32> {
        self.velocity.clone()
    }
}

impl Component for MovementComponent {
    type Storage = DenseVecStorage<Self>;
}
