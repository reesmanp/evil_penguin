use amethyst::{
    animation::InterpolationPrimitive,
    ecs::{System, WriteStorage, ReadStorage, Read, Join},
    core::{Transform, Time, math::Vector3},
    input::{InputHandler, StringBindings},
    assets::AssetStorage,
    renderer::{SpriteSheet, SpriteRender, sprite::TextureCoordinates}
};

use crate::components::{
    core::MovementComponent,
    entities::{
        PenguinComponent,
        PlayerComponent
    }
};

pub struct PenguinMovementSystem;

impl<'a> System<'a> for PenguinMovementSystem {
    type SystemData = (
        ReadStorage<'a, PenguinComponent>,
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, MovementComponent>,
        Read<'a, Time>,
        Read<'a, AssetStorage<SpriteSheet>>
    );

    fn run(&mut self, (penguin, player, sprite_renders, mut transform, mut velocity, time, spritesheet_storage): Self::SystemData) {
        let seconds = time.delta_seconds();
        let (player_transform, _) = (&transform, &player).join().next().unwrap();
        let player_translation = player_transform.translation().clone();
        let (penguin_transform, penguin_velocity, penguin_sprite_render, _) = (&mut transform, &mut velocity, &sprite_renders, &penguin).join().next().unwrap();

        let direction_vector: Vector3<f32> = (player_translation - penguin_transform.translation()).normalize();

        if let Some(penguin_spritesheet) = spritesheet_storage.get(&penguin_sprite_render.sprite_sheet) {
            let penguin_sprite = penguin_spritesheet.sprites.get(0).unwrap();
            penguin_velocity.accelerate(Vector3::new(direction_vector.x, direction_vector.y, 0.0), seconds, penguin_sprite, penguin_transform.scale());
            penguin_transform.prepend_translation_x(penguin_velocity.get_delta_x());
            penguin_transform.prepend_translation_y(penguin_velocity.get_delta_y());
        }
    }
}
