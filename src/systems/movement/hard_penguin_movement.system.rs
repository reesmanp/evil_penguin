use amethyst::{
    assets::AssetStorage,
    core::{Transform, Time, math::Vector3},
    ecs::{System, WriteStorage, ReadStorage, Read, Join, shred::DynamicSystemData},
    prelude::World,
    renderer::{SpriteSheet, SpriteRender}
};

use crate::{
    components::{
        core::MovementComponent,
        entities::{
            CoinComponent,
            PenguinComponent,
            PlayerComponent
        }
    },
    ml::neural_network::NeuralNetwork,
    systems::movement::EntityMovement,
    util::constants::DEFAULT_LEARNING_RATE
};
use std::collections::HashMap;

#[derive(Default)]
pub struct HardPenguinMovementSystem {
    neural_network: Option<NeuralNetwork>,
    pub coin_amount: usize
}

impl<'a> System<'a> for HardPenguinMovementSystem {
    type SystemData = (
        ReadStorage<'a, PenguinComponent>,
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, CoinComponent>,
        ReadStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, MovementComponent>,
        Read<'a, Time>,
        Read<'a, AssetStorage<SpriteSheet>>
    );

    fn run(&mut self, (penguin, player, coins, sprite_renders, mut transform, mut movement, time, sprite_sheet_storage): Self::SystemData) {
        // Find coins that are left
        let mut coin_map = HashMap::new();
        for (coin) in (&coins).join() {
            coin_map.insert(coin.id, 1.0);
        }

        // Mark coins that are not found as 0
        let mut coin_vec = Vec::with_capacity(self.coin_amount);
        for i in 0..self.coin_amount {
            coin_vec.push(*coin_map.get(&i).or_else(|| Some(&0.0)).unwrap());
        }

        if let Some((player_transform, _)) = (&transform, &player).join().next() {
            let player_translation = player_transform.translation().clone();
            let (penguin_transform, penguin_movement, penguin_sprite_render, _) = (&mut transform, &mut movement, &sprite_renders, &penguin).join().next().unwrap();
            let penguin_translation = penguin_transform.translation().clone();
            let mut feature_vec = vec![
                penguin_translation.x,
                penguin_translation.y,
                penguin_movement.velocity.x,
                penguin_movement.velocity.y,
                player_translation.x,
                player_translation.y
            ];
            feature_vec.append(&mut coin_vec);

            if let Some(penguin_sprite_sheet) = sprite_sheet_storage.get(&penguin_sprite_render.sprite_sheet) {
                let penguin_sprite = penguin_sprite_sheet.sprites.get(0).unwrap();
                self.transform_entity(penguin_transform, &feature_vec, &time, penguin_movement, penguin_sprite);
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        let network_structure = vec![4, 16, 2];
        self.neural_network = Some(
            NeuralNetwork::new(network_structure, DEFAULT_LEARNING_RATE, 6 * self.coin_amount)
        );
        <Self::SystemData as DynamicSystemData>::setup(&self.accessor(), world)
    }
}

impl EntityMovement for HardPenguinMovementSystem {
    type AccelerationDirection = (
        Vec<f32>
    );

    fn get_acceleration(&mut self, input: &Self::AccelerationDirection) -> Vector3<f32> {
        let output = self.neural_network.as_mut().unwrap().test(input.clone(), None);
        let x;
        let y;
        if output[0] > 0.5 {
            x = 1.0;
        } else if output[0] < -0.5 {
            x = -1.0;
        } else {
            x = 0.0;
        }
        if output[1] > 0.5 {
            y = 1.0;
        } else if output[1] < -0.5 {
            y = -1.0;
        } else {
            y = 0.0;
        }
        let acceleration = Vector3::new(x, y, 0.0);
        acceleration.normalize() * 5.0
    }
}
