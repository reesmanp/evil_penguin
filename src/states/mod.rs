#[path = "paused.state.rs"]
mod paused_state;
#[path = "run.state.rs"]
mod run_state;
#[path = "start_menu.state.rs"]
mod start_menu_state;

pub use self::{
    paused_state::PausedState,
    run_state::RunState,
    start_menu_state::StartMenuState
};

use crate::{
    util::constants::{
        DEFAULT_ARENA_WIDTH,
        DEFAULT_ARENA_HEIGHT
    }
};

use amethyst::{
    core::Transform,
    prelude::*,
    renderer::{
        Camera
    }
};

pub trait BaseState {
    fn initialize_camera(&self, world: &mut World) {
        // TODO: Prevent multiple cameras from being initialized during state transition
        let mut transform = Transform::default();
        transform.set_translation_xyz(DEFAULT_ARENA_WIDTH / 2.0, DEFAULT_ARENA_HEIGHT / 2.0, 5.0);

        world
            .create_entity()
            .with(transform)
            .with(Camera::standard_2d(DEFAULT_ARENA_WIDTH, DEFAULT_ARENA_HEIGHT))
            .build();
    }
}
