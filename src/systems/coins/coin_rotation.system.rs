use amethyst::{
    core::Time,
    ecs::{System, Read, Join, WriteStorage, Write},
    renderer::SpriteRender
};

use crate::components::entities::CoinComponent;

pub struct CoinRotationSystem;

impl<'a> System<'a> for CoinRotationSystem {
    type SystemData = (
        WriteStorage<'a, CoinComponent>,
        WriteStorage<'a, SpriteRender>,
        Read<'a, Time>
    );

    fn run(&mut self, (mut coins, mut sprites_render, time): Self::SystemData) {
        let delta = time.delta_seconds();
        for (coin, sprite_render) in (&mut coins, &mut sprites_render).join() {
            coin.elapse_time(delta);
            coin.update_frame();
            sprite_render.sprite_number = coin.frame;
        }
    }
}
