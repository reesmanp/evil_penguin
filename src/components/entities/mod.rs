#[path = "coin.component.rs"]
mod coin_component;
#[path = "penguin.component.rs"]
mod penguin_component;
#[path = "player.component.rs"]
mod player_component;

pub use self::{
    coin_component::*,
    penguin_component::*,
    player_component::*
};
