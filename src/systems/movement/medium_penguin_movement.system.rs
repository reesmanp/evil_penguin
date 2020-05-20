use amethyst::{
    assets::AssetStorage,
    core::{Transform, Time, math::Vector3},
    ecs::{System, WriteStorage, ReadStorage, Read, Join},
    input::{InputHandler, StringBindings},
    renderer::{SpriteSheet, SpriteRender}
};

use crate::{
    components::{
        core::MovementComponent,
        entities::{
            PenguinComponent,
            PlayerComponent
        }
    },
    systems::movement::EntityMovement,
};

pub struct MediumPenguinMovementSystem;

impl<'a> System<'a> for MediumPenguinMovementSystem {
    type SystemData = (
        ReadStorage<'a, PenguinComponent>,
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, MovementComponent>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>,
        Read<'a, AssetStorage<SpriteSheet>>
    );

    fn run(&mut self, (penguin, player, sprite_renders, mut transform, mut movement, input, time, sprite_sheet_storage): Self::SystemData) {
        if let Some((player_transform, _)) = (&transform, &player).join().next() {
            let player_translation = player_transform.translation().clone();
            let (penguin_transform, penguin_movement, penguin_sprite_render, _) = (&mut transform, &mut movement, &sprite_renders, &penguin).join().next().unwrap();
            let penguin_translation = penguin_transform.translation().clone();

            if let Some(penguin_sprite_sheet) = sprite_sheet_storage.get(&penguin_sprite_render.sprite_sheet) {
                let penguin_sprite = penguin_sprite_sheet.sprites.get(0).unwrap();

                // Direction of acceleration force vectors, normalized
                let x_direction = match input.axis_value("horizontal") {
                    Some(x) => x as f32,
                    None => 0.0
                };
                let y_direction = match input.axis_value("vertical") {
                    Some(y) => y as f32,
                    None => 0.0
                };
                let player_acceleration = Vector3::new(x_direction, y_direction, 0.0) * 5.0;
                self.transform_entity(penguin_transform, &(player_translation, penguin_translation, player_acceleration), &time, penguin_movement, penguin_sprite);
            }
        }
    }
}

impl EntityMovement for MediumPenguinMovementSystem {
    type AccelerationDirection = (
        Vector3<f32>,
        Vector3<f32>,
        Vector3<f32>
    );

    fn get_acceleration(&self, input: &Self::AccelerationDirection) -> Vector3<f32> {
        // Find vector to actual tip of player acceleration
        let real_player_vector = input.0 + input.2;
        (real_player_vector - input.1).normalize() * 5.0
    }
}
