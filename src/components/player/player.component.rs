use amethyst::{
    ecs::{Component, HashMapStorage}
};

pub struct PlayerComponent;

impl Default for PlayerComponent {
    fn default() -> Self {
        PlayerComponent
    }
}

impl Component for PlayerComponent {
    type Storage = HashMapStorage<Self>;
}
