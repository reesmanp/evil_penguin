use amethyst::{
    ecs::{Component, HashMapStorage}
};

pub struct EndConditionComponent {
    pub is_win: Option<bool>
}

impl Default for EndConditionComponent {
    fn default() -> Self {
        Self {
            is_win: None
        }
    }
}

impl Component for EndConditionComponent {
    type Storage = HashMapStorage<Self>;
}
