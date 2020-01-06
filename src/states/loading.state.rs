use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{
        ImageFormat,
        SpriteSheet,
        SpriteSheetFormat,
        Texture
    }
};

use crate::{
    states::{
        BaseState,
        RunState
    },
    util::types::SpritesheetLoadingData
};

use std::collections::HashMap;

pub enum NextLoadingState {
    Paused,
    Run,
    StartMenu
}

pub struct LoadingState {
    loading_assets: HashMap<String, Handle<SpriteSheet>>,
    next_state: NextLoadingState,
    progress_counter: ProgressCounter
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        match self.next_state {
            NextLoadingState::Paused => {},
            NextLoadingState::Run => {
                let spritesheet_dependencies = RunState::get_dependent_spritesheets();
                for tuple in spritesheet_dependencies {
                    self.load_sprite_sheet(world, tuple)
                }
            },
            NextLoadingState::StartMenu => {}
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            match self.next_state {
                NextLoadingState::Paused => return Trans::None,
                NextLoadingState::Run => {
                    let mut run_state = RunState::default();
                    run_state.set_dependent_spritesheet_handles(&mut self.loading_assets);
                    return Trans::Switch(Box::new(run_state))
                },
                NextLoadingState::StartMenu => return Trans::None
            }
        }

        Trans::None
    }
}

impl LoadingState {
    pub fn new(next_state: NextLoadingState) -> Self {
        Self {
            loading_assets: HashMap::new(),
            next_state,
            progress_counter:ProgressCounter::new()
        }
    }

    fn load_sprite_sheet(
        &mut self,
        world: &mut World,
        (sprite_name, sprite_sheet_path, ron_path): SpritesheetLoadingData
    ) {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                sprite_sheet_path,
                ImageFormat::default(),
                &mut self.progress_counter,
                &texture_storage
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        self.loading_assets.insert(sprite_name.to_string(), loader.load(
            ron_path,
            SpriteSheetFormat(texture_handle),
            &mut self.progress_counter,
            &sprite_sheet_store
        ));
    }
}
