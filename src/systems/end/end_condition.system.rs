use amethyst::{
    ecs::{System, WriteStorage, ReadStorage, Join, Read},
    core::Transform,
    assets::AssetStorage,
    renderer::{SpriteSheet, SpriteRender},
};
use crate::{
    components::{
        core::EndConditionComponent,
        entities::{
            PenguinComponent,
            PlayerComponent
        }
    },
    util::{
        get_sprite_coordinates,
        is_collision
    }
};

pub struct EndConditionSystem;

impl<'a> System<'a> for EndConditionSystem {
    type SystemData = (
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, PenguinComponent>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, SpriteRender>,
        WriteStorage<'a, EndConditionComponent>,
        Read<'a, AssetStorage<SpriteSheet>>
    );

    fn run(&mut self, (player, penguin, transform, sprite_renders, mut end_condition, spritesheet_storage): Self::SystemData) {
        let (player_transform, player_sprite_render, _) = (&transform, &sprite_renders, &player).join().next().unwrap();
        let (penguin_transform, penguin_sprite_render, _) = (&transform, &sprite_renders, &penguin).join().next().unwrap();
        let mut end_condition_instance = (&mut end_condition).join().next().unwrap();

        // Get spritesheets
        if let (Some(player_spritesheet), Some(penguin_spritesheet)) = (
            spritesheet_storage.get(&player_sprite_render.sprite_sheet),
            spritesheet_storage.get(&penguin_sprite_render.sprite_sheet)
        ) {
            // Get actual sprites used. These spritesheets only have 1 sprite in them.
            let player_sprite = player_spritesheet.sprites.get(0).unwrap();
            let penguin_sprite = penguin_spritesheet.sprites.get(0).unwrap();

            // Get entity coordinates
            let player_coords = get_sprite_coordinates(player_transform, player_sprite);
            let penguin_coords = get_sprite_coordinates(penguin_transform, penguin_sprite);

            if is_collision(player_coords, penguin_coords) {
                end_condition_instance.is_win = Some(false);
            }
        }
    }
}
