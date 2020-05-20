#[path = "lose_menu.state.rs"]
mod lose_menu_state;
#[path = "main_menu.state.rs"]
mod main_menu_state;
#[path = "start_menu.state.rs"]
mod start_menu_state;
#[path = "win_menu.state.rs"]
mod win_menu_state;

pub use self::{
    lose_menu_state::LoseMenuState,
    main_menu_state::MainMenuState,
    start_menu_state::StartMenuState,
    win_menu_state::WinMenuState
};

use crate::{
    util::constants::{
        DEFAULT_ARENA_WIDTH,
        DEFAULT_ARENA_HEIGHT,
        SQUARE_FONT_PATH
    }
};

use amethyst::{
    assets::{Loader, ProgressCounter},
    core::ArcThreadPool,
    ecs::{Dispatcher, DispatcherBuilder, Entity},
    prelude::{World, WorldExt},
    ui::{Anchor, FontHandle, TtfFormat, UiCreator, UiTransform}
};

pub trait BaseMenuState<'a, 'b> {
    fn get_progress_counter(&mut self) -> &mut ProgressCounter;

    fn initialize_dispatcher(&self, world: &mut World, dispatcher_builder: DispatcherBuilder<'a, 'b>) -> Option<Dispatcher<'a, 'b>> {
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        Some(dispatcher)
    }

    fn initialize_font(&mut self, world: &mut World) -> Option<FontHandle> {
        Some(world.read_resource::<Loader>().load(
            SQUARE_FONT_PATH,
            TtfFormat,
            self.get_progress_counter(),
            &world.read_resource(),
        ))
    }

    fn initialize_menu(&mut self, world: &mut World, ron_file: &str) -> Option<Entity> {
        Some(world.exec(|mut creator: UiCreator<'_>| {
            creator.create(ron_file, self.get_progress_counter())
        }))
    }

    fn create_ui_transform(&self, id: String, x: f32, y: f32, z: f32) -> UiTransform {
        UiTransform::new(
            id,
            Anchor::TopMiddle,
            Anchor::TopMiddle,
            x,
            y,
            z,
            DEFAULT_ARENA_WIDTH / 2.0,
            DEFAULT_ARENA_HEIGHT / 2.0
        )
    }
}
