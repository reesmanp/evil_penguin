#[path = "loading.state.rs"]
mod loading_state;
#[path = "paused.state.rs"]
mod paused_state;
#[path = "run.state.rs"]
mod run_state;

pub mod menu;

pub use self::{
    loading_state::{LoadingState, NextLoadingState},
    paused_state::PausedState,
    run_state::RunState,
};

use crate::{
    util::{
        constants::{
            DEFAULT_ARENA_WIDTH,
            DEFAULT_ARENA_HEIGHT
        },
        types::SpriteSheetLoadingData
    }
};

use amethyst::{
    assets::Handle,
    core::Transform,
    prelude::*,
    renderer::{
        Camera,
        SpriteSheet
    }
};
use std::collections::HashMap;

pub trait BaseState {
    fn initialize_camera(&self, world: &mut World) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(DEFAULT_ARENA_WIDTH / 2.0, DEFAULT_ARENA_HEIGHT / 2.0, 5.0);

        world
            .create_entity()
            .with(transform)
            .with(Camera::standard_2d(DEFAULT_ARENA_WIDTH, DEFAULT_ARENA_HEIGHT))
            .build();
    }

    fn get_dependent_spritesheets() -> Vec<SpriteSheetLoadingData<'static>> {
        vec![]
    }

    fn set_dependent_spritesheet_handles(&mut self, _handle_map: &mut HashMap<String, Handle<SpriteSheet>>) {
    }
}
