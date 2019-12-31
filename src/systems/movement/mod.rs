#[path = "penguin_movement.system.rs"]
mod penguin_system;
#[path = "player_movement.system.rs"]
mod player_movement_system;

pub use self::{
    penguin_system::*,
    player_movement_system::*
};
