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
        PausedState,
        RunState,
        menu::{
            LoseMenuState,
            MainMenuState,
            StartMenuState,
            WinMenuState
        }
    },
    systems::movement::Difficulty,
    util::types::SpritesheetLoadingData
};

use std::collections::HashMap;

pub enum NextLoadingState {
    Paused,
    UnPaused,
    Run(Difficulty),
    MainMenu,
    StartMenu,
    EndMenu(bool)
}

pub struct LoadingState {
    loading_assets: HashMap<String, Handle<SpriteSheet>>,
    next_state: NextLoadingState,
    progress_counter: ProgressCounter
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_dependencies = match self.next_state {
            NextLoadingState::Paused => PausedState::get_dependent_spritesheets(),
            NextLoadingState::Run(_) => RunState::get_dependent_spritesheets(),
            NextLoadingState::StartMenu => StartMenuState::get_dependent_spritesheets(),
            NextLoadingState::EndMenu(is_win) => match is_win {
                true => WinMenuState::get_dependent_spritesheets(),
                false => LoseMenuState::get_dependent_spritesheets()
            },
            NextLoadingState::MainMenu => MainMenuState::get_dependent_spritesheets(),
            _ => vec![]
        };

        for tuple in sprite_sheet_dependencies {
            self.load_sprite_sheet(world, tuple)
        }
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            return match &self.next_state {
                NextLoadingState::Paused => Trans::Switch(Box::new(PausedState::default())),
                NextLoadingState::UnPaused => Trans::Pop,
                NextLoadingState::Run(difficulty) => {
                    let mut run_state = RunState::default();
                    run_state.set_difficulty((*difficulty).clone());
                    run_state.set_dependent_spritesheet_handles(&mut self.loading_assets);
                    Trans::Switch(Box::new(run_state))
                },
                NextLoadingState::StartMenu => Trans::None,
                NextLoadingState::EndMenu(is_win) => {
                    if *is_win {
                        return Trans::Switch(Box::new(WinMenuState::default()));
                    }

                    Trans::Switch(Box::new(LoseMenuState::default()))
                },
                NextLoadingState::MainMenu => Trans::Switch(Box::new(MainMenuState::default()))
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
