use amethyst::{
    ecs::{System, WriteStorage, ReadStorage, Write, Join, Read, Entities},
    core::Transform,
    assets::AssetStorage,
    renderer::{SpriteSheet, SpriteRender, sprite::TextureCoordinates}
};
use crate::{
    components::entities::{
        CoinComponent,
        PlayerComponent
    },
    util::{
        get_sprite_coordinates,
        is_collision
    }
};

pub struct CoinCollectionSystem;

impl<'a> System<'a> for CoinCollectionSystem {
    type SystemData = (
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, CoinComponent>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, SpriteRender>,
        Read<'a, AssetStorage<SpriteSheet>>,
        Entities<'a>
    );

    fn run(&mut self, (player, coins, transform, sprite_renders, spritesheet_storage, entities): Self::SystemData) {
        let (player_transform, player_sprite_render, _) = (&transform, &sprite_renders, &player).join().next().unwrap();
        let (coin_sprite_render, _) = (&sprite_renders, &coins).join().next().unwrap();

        // Get spritesheets
        if let (Some(player_spritesheet), Some(coin_spritesheet)) = (
            spritesheet_storage.get(&player_sprite_render.sprite_sheet),
            spritesheet_storage.get(&coin_sprite_render.sprite_sheet)
        ) {
            for (coin_entity, coin_transform, _) in (&*entities, &transform, &coins).join() {
                // Get actual sprites used. These spritesheets only have 1 sprite in them.
                let player_sprite = player_spritesheet.sprites.get(0).unwrap();
                let coin_sprite = coin_spritesheet.sprites.get(0).unwrap();

                // Get entity coordinates
                let player_coords = get_sprite_coordinates(player_transform, player_sprite);
                let coin_coords = get_sprite_coordinates(coin_transform, coin_sprite);

                if is_collision(player_coords, coin_coords) {
                    entities.delete(coin_entity);
                }
            }
        }
    }
}

