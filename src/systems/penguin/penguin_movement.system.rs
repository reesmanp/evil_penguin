use amethyst::{
    animation::InterpolationPrimitive,
    ecs::{System, WriteStorage, ReadStorage, Read, Join},
    core::{Transform, Time, math::base::Vector3}
};

use crate::components::{
    core::VelocityComponent,
    penguin::PenguinComponent,
    player::PlayerComponent
};

pub struct PenguinMovementSystem;

impl<'a> System<'a> for PenguinMovementSystem {
    type SystemData = (
        ReadStorage<'a, PenguinComponent>,
        ReadStorage<'a, PlayerComponent>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, VelocityComponent>,
        Read<'a, Time>
    );

    fn run(&mut self, (penguin, player, mut transform, mut velocity, time): Self::SystemData) {
        let seconds = time.delta_seconds();
        let (player_transform, _) = (&transform, &player).join().next().unwrap();
        let player_translation = player_transform.translation().clone();
        let (penguin_transform, penguin_velocity, _) = (&mut transform, &mut velocity, &penguin).join().next().unwrap();

        let direction_vector: Vector3<f32> = (player_translation - penguin_transform.translation()).normalize();
        let mut acceleration = Vector3::new(
            direction_vector.x * penguin_velocity.acceleration_magnitude,
            direction_vector.y * penguin_velocity.acceleration_magnitude,
            0.0
        );

        if acceleration.x == 0.0 {
            acceleration.x = 0.0 * penguin_velocity.x.normalize() * penguin_velocity.coast_magnitude;
        }
        if acceleration.y == 0.0 {
            acceleration.y = 0.0 * penguin_velocity.y.normalize() * penguin_velocity.coast_magnitude;
        }

        let mut new_penguin_velocity = Vector3::new(
            penguin_velocity.x + acceleration.x * seconds,
            penguin_velocity.y + acceleration.y * seconds,
            0.0
        );

        let current_speed = penguin_velocity.x.abs() + penguin_velocity.y.abs();
        let new_speed = new_penguin_velocity.magnitude();
        if new_speed > penguin_velocity.max_speed {
            new_penguin_velocity.x = penguin_velocity.x + (direction_vector.x * (penguin_velocity.max_speed - current_speed));
            new_penguin_velocity.y = penguin_velocity.y + (direction_vector.y * (penguin_velocity.max_speed - current_speed));
        }

        penguin_transform.prepend_translation_x((penguin_velocity.x + new_penguin_velocity.x) * seconds / 2.0);
        penguin_transform.prepend_translation_y((penguin_velocity.y + new_penguin_velocity.y) * seconds / 2.0);

        penguin_velocity.x = new_penguin_velocity.x;
        penguin_velocity.y = new_penguin_velocity.y;
    }
}
