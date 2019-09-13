use amethyst::{
    ecs::{Component, HashMapStorage}
};

pub struct PenguinComponent;

impl Default for PenguinComponent {
    fn default() -> Self {
        Self
    }
}

impl Component for PenguinComponent {
    type Storage = HashMapStorage<Self>;
}
