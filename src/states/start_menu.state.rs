use amethyst::{
    assets::{Loader, ProgressCounter},
    core::ArcThreadPool,
    ecs::{Dispatcher, DispatcherBuilder},
    input::{VirtualKeyCode, is_key_down},
    prelude::*,
    ui::{UiTransform, Anchor, UiText, TtfFormat, FontHandle}
};

use crate::{
    states::{
        BaseState,
        Menu,
        RunState
    },
    systems::menu::TitleBlinkSystem,
    util::constants::{DEFAULT_ARENA_WIDTH, DEFAULT_ARENA_HEIGHT, SQUARE_FONT_PATH}
};

use std::collections::HashMap;

enum FontSize {
    SMALL = 10,
    MEDIUM = 20,
    LARGE = 35,
    TITLE = 50
}

#[derive(Default)]
pub struct StartMenuState<'a, 'b> {
    font: Option<FontHandle>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    progress_counter: ProgressCounter
}

impl<'a, 'b> SimpleState for StartMenuState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.initialize_dispatcher(world);
        self.initialize_font(world);
        self.initialize_start_menu(world);
        self.initialize_camera(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(e) = &event {
            if is_key_down(&e, VirtualKeyCode::Return) || is_key_down(&e, VirtualKeyCode::NumpadEnter) {
                return Trans::Switch(Box::new(RunState::default()));
            }
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        Trans::None
    }
}

impl<'a, 'b> StartMenuState<'a, 'b> {
    fn initialize_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(TitleBlinkSystem, "title_blink_system", &[]);

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

    fn initialize_start_menu(&self, world: &mut World) {
        let title_transform = self.create_ui_transform("title_entity".to_string(), 0.0, 100.0, 1.0);
        let title_text = self.create_ui_text("Evil Penguin".to_string(), FontSize::TITLE);

        let continue_transform = self.create_ui_transform("continue_entity".to_string(), 0.0, 0.0, 1.0);
        let continue_text = self.create_ui_text("<< Press ENTER to continue >>".to_string(), FontSize::MEDIUM);

        let title = world
            .create_entity()
            .with(title_transform)
            .with(title_text)
            .build();

        let text = world
            .create_entity()
            .with(continue_transform)
            .with(continue_text)
            .build();

        let mut text_map = HashMap::new();
        text_map.insert("continue_text".to_string(), text);

        world.insert(Menu {
            id: "title_menu".to_string(),
            title: Some(title),
            text: Some(text_map),
            buttons: None
        })
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

impl<'a, 'b> BaseState for StartMenuState<'a,'b> {}
