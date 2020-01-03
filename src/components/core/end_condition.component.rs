use amethyst::{
    ecs::{Component, HashMapStorage}
};

#[derive(Component, Default)]
#[storage(HashMapStorage)]
pub struct EndConditionComponent {
    pub is_win: Option<bool>
}
