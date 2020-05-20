use amethyst::{
    assets::ProgressCounter,
    ecs::{Dispatcher, DispatcherBuilder},
    prelude::*,
    ui::{FontHandle, UiFinder, UiEventType}
};

use crate::{
    states::{
        BaseState,
        LoadingState,
        NextLoadingState,
        menu::BaseMenuState
    },
    systems::movement::Difficulty,
    util::constants::MAIN_MENU_RON_PATH
};

#[derive(Default)]
pub struct MainMenuState<'a, 'b> {
    font: Option<FontHandle>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    progress_counter: ProgressCounter
}

impl<'a, 'b> SimpleState for MainMenuState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dispatcher_builder = DispatcherBuilder::new();

        self.dispatcher = self.initialize_dispatcher(world, dispatcher_builder);
        self.font = self.initialize_font(world);
        self.initialize_menu(world, MAIN_MENU_RON_PATH);
        self.initialize_camera(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Ui(ui_event) = &event {
            if ui_event.event_type == UiEventType::Click {
                let (easy, medium, hard) = data.world.exec(|ui_finder: UiFinder<'_>| {
                    (ui_finder.find("play_easy"), ui_finder.find("play_medium"), ui_finder.find("play_hard"))
                });

                if Some(ui_event.target) == easy {
                    return Trans::Switch(Box::new(LoadingState::new(NextLoadingState::Run(Difficulty::Easy))))
                } else if Some(ui_event.target) == medium {
                    return Trans::Switch(Box::new(LoadingState::new(NextLoadingState::Run(Difficulty::Medium))))
                } else if Some(ui_event.target) == hard {
                    return Trans::Switch(Box::new(LoadingState::new(NextLoadingState::Run(Difficulty::Hard))))
                }
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

impl<'a, 'b> BaseState for MainMenuState<'a,'b> {}

impl<'a, 'b, 'c> BaseMenuState<'_, '_> for MainMenuState<'a, 'b> {
    fn get_progress_counter(&mut self) -> &mut ProgressCounter {
        &mut self.progress_counter
    }
}
