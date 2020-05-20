use amethyst::{
    ecs::{System, WriteStorage, ReadStorage, Read, Join},
    core::{Transform, Time, math::Vector3},
    assets::AssetStorage,
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
    systems::movement::EntityMovement
};

pub struct EasyPenguinMovementSystem;

impl<'a> System<'a> for EasyPenguinMovementSystem {
    type SystemData = (
        ReadStorage<'a, PenguinComponent>,
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, MovementComponent>,
        Read<'a, Time>,
        Read<'a, AssetStorage<SpriteSheet>>
    );

    fn run(&mut self, (penguin, player, sprite_renders, mut transform, mut movement, time, sprite_sheet_storage): Self::SystemData) {
        if let Some((player_transform, _)) = (&transform, &player).join().next() {
            let player_translation = player_transform.translation().clone();
            let (penguin_transform, penguin_movement, penguin_sprite_render, _) = (&mut transform, &mut movement, &sprite_renders, &penguin).join().next().unwrap();
            let penguin_translation = penguin_transform.translation().clone();

            if let Some(penguin_sprite_sheet) = sprite_sheet_storage.get(&penguin_sprite_render.sprite_sheet) {
                let penguin_sprite = penguin_sprite_sheet.sprites.get(0).unwrap();
                self.transform_entity(penguin_transform, &(player_translation, penguin_translation), &time, penguin_movement, penguin_sprite);
            }
        }
    }
}

impl EntityMovement for EasyPenguinMovementSystem {
    type AccelerationDirection = (
        Vector3<f32>,
        Vector3<f32>
    );

    fn get_acceleration(&self, input: &Self::AccelerationDirection) -> Vector3<f32> {
        (input.0 - input.1).normalize() * 5.0
    }
}
