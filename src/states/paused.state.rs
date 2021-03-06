use amethyst::{
    assets::ProgressCounter,
    ecs::{Dispatcher, DispatcherBuilder, Entity},
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
    util::constants::PAUSED_MENU_RON_PATH
};

#[derive(Default)]
pub struct PausedState<'a, 'b> {
    container_entity: Option<Entity>,
    font: Option<FontHandle>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    progress_counter: ProgressCounter
}

impl<'a, 'b> SimpleState for PausedState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(MenuBlinkSystem, "win_blink_system", &[]);

        self.dispatcher = self.initialize_dispatcher(world, dispatcher_builder);
        self.font = self.initialize_font(world);
        self.container_entity = self.initialize_menu(world, PAUSED_MENU_RON_PATH);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(entity) = self.container_entity {
            data.world.delete_entity(entity).unwrap();
            self.container_entity = None;
        }
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(e) = &event {
            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Switch(Box::new(LoadingState::new(NextLoadingState::UnPaused)));
            }
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            if let Some(dispatcher) = self.dispatcher.as_mut() {
                dispatcher.dispatch(&data.world);
            }
        }

        Trans::None
    }
}

impl<'a, 'b> BaseState for PausedState<'a,'b> {
}

impl<'a, 'b> BaseMenuState<'_, '_> for PausedState<'a, 'b> {
    fn get_progress_counter(&mut self) -> &mut ProgressCounter {
        &mut self.progress_counter
    }
}
