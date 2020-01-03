use amethyst::{
    ecs::{Component, HashMapStorage}
};

#[derive(Component, Default)]
#[storage(HashMapStorage)]
pub struct PlayerComponent;
