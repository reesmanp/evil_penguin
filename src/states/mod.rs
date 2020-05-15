#[path = "loading.state.rs"]
mod loading_state;
#[path = "lose_menu.state.rs"]
mod lose_menu_state;
#[path = "paused.state.rs"]
mod paused_state;
#[path = "run.state.rs"]
mod run_state;
#[path = "start_menu.state.rs"]
mod start_menu_state;
#[path = "win_menu.state.rs"]
mod win_menu_state;

pub use self::{
    loading_state::{LoadingState, NextLoadingState},
    lose_menu_state::LoseMenuState,
    paused_state::PausedState,
    run_state::RunState,
    start_menu_state::StartMenuState,
    start_menu_state::MenuEntities,
    win_menu_state::WinMenuState
};

use crate::{
    util::{
        constants::{
            DEFAULT_ARENA_WIDTH,
            DEFAULT_ARENA_HEIGHT
        },
        types::SpritesheetLoadingData
    }
};

use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{
        Camera,
        SpriteSheet
    }
};
use std::collections::HashMap;

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

    fn get_dependent_spritesheets() -> Vec<SpritesheetLoadingData<'static>> {
        vec![]
    }

    fn set_dependent_spritesheet_handles(&mut self, handle_map: &mut HashMap<String, Handle<SpriteSheet>>) {
    }
}
