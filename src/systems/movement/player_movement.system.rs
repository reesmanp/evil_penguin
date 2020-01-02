use amethyst::{
    animation::InterpolationPrimitive,
    ecs::{System, WriteStorage, ReadStorage, Read, Join},
    core::{Transform, Time, math::Vector3},
    input::{InputHandler, StringBindings},
    assets::AssetStorage,
    renderer::{SpriteSheet, SpriteRender, sprite::TextureCoordinates, Sprite}
};

use crate::{
    components::{
        core::MovementComponent,
        entities::PlayerComponent
    },
    systems::movement::EntityMovement
};

pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, MovementComponent>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>,
        Read<'a, AssetStorage<SpriteSheet>>
    );

    fn run(&mut self, (player, sprite_renders, mut transform, mut movement, input, time, spritesheet_storage): Self::SystemData) {
        let (player_transform, player_movement, player_sprite_render, _) = (&mut transform, &mut movement, &sprite_renders, &player).join().next().unwrap();

        // Time since last frame
        let seconds = time.delta_seconds();

        if let Some(spritesheet) = spritesheet_storage.get(&player_sprite_render.sprite_sheet) {
            let player_sprite = spritesheet.sprites.get(0).unwrap();
            self.transform_entity(player_transform, &input, &time, player_movement, player_sprite);
        }
    }
}

impl EntityMovement for PlayerMovementSystem {
    type AccelerationDirection = InputHandler<StringBindings>;

    fn get_acceleration(&self, input: &Self::AccelerationDirection) -> Vector3<f32> {
        // Direction of acceleration force vectors, normalized
        let x_direction = match input.axis_value("horizontal") {
            Some(x) => x as f32,
            None => 0.0
        };
        let y_direction = match input.axis_value("vertical") {
            Some(y) => y as f32,
            None => 0.0
        };

        Vector3::new(x_direction, y_direction, 0.0) * 5.0
    }
}
