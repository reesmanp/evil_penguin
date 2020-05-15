use amethyst::{
    assets::{Loader, ProgressCounter, Handle},
    core::ArcThreadPool,
    ecs::{Dispatcher, DispatcherBuilder, Entity},
    input::{VirtualKeyCode, is_key_down},
    prelude::*,
    renderer::SpriteSheet,
    ui::{UiTransform, Anchor, UiText, TtfFormat, FontHandle, UiCreator, UiFinder}
};

use crate::{
    states::{
        BaseState,
        LoadingState,
        NextLoadingState,
    },
    systems::menu::MenuBlinkSystem,
    util::{
        constants::{DEFAULT_ARENA_WIDTH, DEFAULT_ARENA_HEIGHT, SQUARE_FONT_PATH},
        types::SpritesheetLoadingData
    }
};

use std::collections::HashMap;
use crate::states::MenuEntities;

enum FontSize {
    SMALL = 10,
    MEDIUM = 20,
    LARGE = 35,
    TITLE = 50
}

#[derive(Default)]
pub struct LoseMenuState<'a, 'b> {
    font: Option<FontHandle>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    progress_counter: ProgressCounter
}

impl<'a, 'b> SimpleState for LoseMenuState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.initialize_dispatcher(world);
        self.initialize_font(world);
        self.initialize_menu(world);
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

impl<'a, 'b> LoseMenuState<'a, 'b> {
    fn initialize_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(MenuBlinkSystem, "menu_blink_system", &[]);

        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }

    fn initialize_font(&mut self, world: &mut World) {
        self.font = Some(world.read_resource::<Loader>().load(
            SQUARE_FONT_PATH,
            TtfFormat,
            (),
            &world.read_resource(),
        ));
    }

    fn initialize_menu(&self, world: &mut World) {
        let (top_level_entity, progress_counter) = world.exec(|mut creator: UiCreator<'_>| {
            let mut progress_counter = ProgressCounter::new();
            let main_prefab_entity = creator.create("ui_layouts/lose_menu.ron", &mut progress_counter);
            (main_prefab_entity, progress_counter)
        });
        let mut top_level_entities = HashMap::new();
        top_level_entities.insert("title".to_string(), top_level_entity);
        let essential_entities = HashMap::new();
        let mut progress_counters = HashMap::new();
        progress_counters.insert("text".to_string(), progress_counter);
        world.insert(MenuEntities {
            top_level_entities,
            essential_entities,
            progress_counters
        });
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

    fn create_ui_text(&self, text: String, size: FontSize) -> UiText {
        UiText::new(
            self.font.as_ref().unwrap().clone(),
            text,
            [1., 1., 1., 1.],
            size as i32 as f32
        )
    }
}

impl<'a, 'b> BaseState for LoseMenuState<'a,'b> {
}
