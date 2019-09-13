use amethyst::{
    animation::InterpolationPrimitive,
    ecs::{System, WriteStorage, ReadStorage, Read, Join},
    core::{Transform, Time},
    input::{InputHandler, StringBindings}
};

use crate::components::{
    core::VelocityComponent,
    player::PlayerComponent
};

pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, VelocityComponent>,
        ReadStorage<'a, PlayerComponent>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>
    );

    fn run(&mut self, (mut transform, mut velocity, player, input, time): Self::SystemData) {
        for (local_transform, local_velocity, _) in (&mut transform, &mut velocity, &player).join() {
            // Time since last frame
            let seconds = time.delta_seconds();

            // Direction of acceleration force vectors, normalized
            let x_acceleration_dir = match input.axis_value("horizontal") {
                Some(x) => x as f32,
                None => 0.0
            };
            let y_acceleration_dir = match input.axis_value("vertical") {
                Some(y) => y as f32,
                None => 0.0
            };

            // Initialize new velocity component magnitudes
            let mut new_x = 0.0;
            let mut new_y = 0.0;

            // Find percentage of x direction sub-component of acceleration to normalize sub-component magnitudes
            let mut percent_x_dir = 0.0;
            if x_acceleration_dir != 0.0 || y_acceleration_dir != 0.0 {
                percent_x_dir = x_acceleration_dir.abs() / (x_acceleration_dir.abs() + y_acceleration_dir.abs());
            }

            // Initialize force component vectors based off of magnitude and direction
            let mut x_acceleration = local_velocity.acceleration_magnitude * x_acceleration_dir * percent_x_dir;
            let mut y_acceleration = local_velocity.acceleration_magnitude * y_acceleration_dir * (1.0 - percent_x_dir);

            // If no acceleration in a given direction, it begins to coast
            if x_acceleration == 0.0 {
                x_acceleration = -1.0 * local_velocity.x.normalize() * local_velocity.coast_magnitude;
            }
            if y_acceleration == 0.0 {
                y_acceleration = -1.0 * local_velocity.y.normalize() * local_velocity.coast_magnitude;
            }

            // Calculate v1
            // v1 = v0 + a * t
            new_x = local_velocity.x + x_acceleration * seconds;
            new_y = local_velocity.y + y_acceleration * seconds;

            // If new speed is greater than the max speed, lower the new values proportionally to meet max speed
            let new_speed = new_x.abs() + new_y.abs();
            let current_speed = local_velocity.x.abs() + local_velocity.y.abs();
            if new_speed > local_velocity.max_speed {
                new_x = local_velocity.x + (percent_x_dir * (local_velocity.max_speed - current_speed));
                new_y = local_velocity.y + ((1.0 - percent_x_dir) * (local_velocity.max_speed - current_speed));
            }

            // Calculate new relative position
            // (v0 + v1) * t / 2
            local_transform.prepend_translation_x((local_velocity.x + new_x) * seconds / 2.0);
            local_transform.prepend_translation_y((local_velocity.y + new_y) * seconds / 2.0);

            // Update velocity with new values
            local_velocity.x = new_x;
            local_velocity.y = new_y;
        }
    }
}
