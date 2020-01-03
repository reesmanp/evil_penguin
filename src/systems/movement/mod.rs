#[path = "penguin_movement.system.rs"]
mod penguin_system;
#[path = "player_movement.system.rs"]
mod player_movement_system;

pub use self::{
    penguin_system::*,
    player_movement_system::*
};

use crate::{
    components::core::MovementComponent,
    util::constants::{
        DEFAULT_ARENA_HEIGHT,
        DEFAULT_ARENA_WIDTH
    }
};

use amethyst::{
    core::{
        math::Vector3,
        Time,
        Transform
    },
    renderer::Sprite
};

pub trait EntityMovement {
    type AccelerationDirection;

    fn get_acceleration(&self, input: &Self::AccelerationDirection) -> Vector3<f32>;

    fn calculate_new_velocity(&self, velocity: &Vector3<f32>, acceleration: &Vector3<f32>, friction: &Vector3<f32>, max_speed: f32) -> Vector3<f32> {
        let mut new_velocity = velocity.clone();

        if new_velocity.magnitude() > max_speed {
            return new_velocity;
        }

        if acceleration.x == 0.0 {
            new_velocity.x += friction.x;
            if new_velocity.x.signum() == friction.x.signum() {
                new_velocity.x = 0.0;
            }
        } else {
            new_velocity.x += acceleration.x;
        }

        if acceleration.y == 0.0 {
            new_velocity.y += friction.y;
            if new_velocity.y.signum() == friction.y.signum() {
                new_velocity.y = 0.0;
            }
        } else {
            new_velocity.y += acceleration.y;
        }

        new_velocity
    }

    fn crash(&self, local_transform: &mut Transform, velocity: Vector3<f32>, sprite: &Sprite) -> Vector3<f32> {
        let mut crashed_velocity = velocity.clone();

        // Crash Scenario
        if local_transform.translation().x < sprite.width * 0.5 * local_transform.scale().x {
            println!("{:?} {:?}", local_transform.translation(), sprite.width);
            local_transform.translation_mut().x = sprite.width * 0.5 * local_transform.scale().x;
            crashed_velocity.x = 0.0;
        } else if local_transform.translation().x + sprite.width * 0.5 * local_transform.scale().x > DEFAULT_ARENA_WIDTH {
            local_transform.translation_mut().x = DEFAULT_ARENA_WIDTH - sprite.width * 0.5 * local_transform.scale().x;
            crashed_velocity.x = 0.0;
        }

        // Crash Scenario
        if local_transform.translation().y < sprite.height * 0.5 * local_transform.scale().y {
            local_transform.translation_mut().y = sprite.height * 0.5 * local_transform.scale().y;
            crashed_velocity.y = 0.0;
        } else if local_transform.translation().y + sprite.height * 0.5 * local_transform.scale().y > DEFAULT_ARENA_HEIGHT {
            local_transform.translation_mut().y = DEFAULT_ARENA_HEIGHT - sprite.height * 0.5 * local_transform.scale().y;
            crashed_velocity.y = 0.0;
        }

        crashed_velocity
    }

    fn get_friction_direction_vector(&self, acceleration: &Vector3<f32>, friction: f32) -> Vector3<f32> {
        let mut friction_vector = Vector3::new(friction, friction, 0.0);
        friction_vector.x = -1.0 * friction.copysign(acceleration.x);
        friction_vector.y = -1.0 * friction.copysign(acceleration.y);
        friction_vector
    }

    fn transform_entity(&self, local_transform: &mut Transform, input: &Self::AccelerationDirection, time: &Time, movement: &mut MovementComponent, sprite: &Sprite) {
        let acceleration = self.get_acceleration(input);
        let friction_vector = self.get_friction_direction_vector(&acceleration, movement.friction);
        let new_velocity = self.calculate_new_velocity(&movement.velocity, &acceleration, &friction_vector, movement.max_speed);
        self.update_transform(local_transform, new_velocity, time);
        let crashed_velocity = self.crash(local_transform, new_velocity, sprite);
        movement.velocity = crashed_velocity;
    }

    fn update_transform(&self, local_transform: &mut Transform, velocity: Vector3<f32>, time: &Time) {
        let old_position = local_transform.translation();
        let new_position = local_transform.translation() + velocity * time.delta_seconds();
        local_transform.prepend_translation(new_position - old_position);
    }
}
