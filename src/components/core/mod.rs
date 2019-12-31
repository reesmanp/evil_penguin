#[path = "end_condition.component.rs"]
mod end_condition_component;
#[path = "movement.component.rs"]
mod movement_component;

pub use self::{
    end_condition_component::*,
    movement_component::*
};

