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
    entities::PlayerComponent
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

    fn run(&mut self, (player, sprite_renders, mut transform, mut velocity, input, time, spritesheet_storage): Self::SystemData) {
        let (player_transform, player_velocity, player_sprite_render, _) = (&mut transform, &mut velocity, &sprite_renders, &player).join().next().unwrap();

        // Time since last frame
        let seconds = time.delta_seconds();

        // Direction of acceleration force vectors, normalized
        let x_direction = match input.axis_value("horizontal") {
            Some(x) => x as f32,
            None => 0.0
        };
        let y_direction = match input.axis_value("vertical") {
            Some(y) => y as f32,
            None => 0.0
        };

        if let Some(spritesheet) = spritesheet_storage.get(&player_sprite_render.sprite_sheet) {
            let player_sprite = spritesheet.sprites.get(0).unwrap();
            player_velocity.accelerate(Vector3::new(x_direction, y_direction, 0.0), seconds, player_sprite, player_transform.scale());
            let velocity = player_velocity.get_velocity_vector();
            player_transform.prepend_translation_x(player_velocity.get_delta_x());
            player_transform.prepend_translation_y(player_velocity.get_delta_y());
            if player_transform.translation().x != player_velocity.position.x || player_transform.translation().y != player_velocity.position.y {
                println!("{:?} {:?}", player_transform.translation(), player_velocity.position);
            }
        }
    }
}
