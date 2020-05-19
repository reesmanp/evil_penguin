use amethyst::{
    assets::ProgressCounter,
    ecs::{Dispatcher, DispatcherBuilder},
    input::{VirtualKeyCode, is_key_down},
    prelude::*,
    ui::FontHandle
};

use crate::{
    states::{
        BaseState,
        LoadingState,
        NextLoadingState,
        menu::BaseMenuState
    },
    systems::menu::MenuBlinkSystem,
    util::constants::TITLE_MENU_RON_PATH
};

#[derive(Default)]
pub struct StartMenuState<'a, 'b> {
    font: Option<FontHandle>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    progress_counter: ProgressCounter
}

impl<'a, 'b> SimpleState for StartMenuState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(MenuBlinkSystem, "title_blink_system", &[]);

        self.dispatcher = self.initialize_dispatcher(world, dispatcher_builder);
        self.font = self.initialize_font(world);
        self.initialize_menu(world, TITLE_MENU_RON_PATH);
        self.initialize_camera(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(e) = &event {
            if is_key_down(&e, VirtualKeyCode::Return) || is_key_down(&e, VirtualKeyCode::NumpadEnter) {
                return Trans::Switch(Box::new(LoadingState::new(NextLoadingState::Run)));
            }
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
//        let title_text = ui_finder.find("title_text").unwrap();
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        Trans::None
    }
}

impl<'a, 'b> BaseState for StartMenuState<'a,'b> {}

impl<'a, 'b, 'c> BaseMenuState<'_, '_> for StartMenuState<'a, 'b> {
    fn get_progress_counter(&mut self) -> &mut ProgressCounter {
        &mut self.progress_counter
    }
}
