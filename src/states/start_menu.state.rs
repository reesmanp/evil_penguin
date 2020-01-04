use amethyst::{
    assets::{Loader},
    prelude::*,
    ui::{UiTransform, Anchor, UiText, TtfFormat, FontHandle}
};

use crate::{
    states::{
        BaseState
    },
    util::constants::{DEFAULT_ARENA_WIDTH, DEFAULT_ARENA_HEIGHT, SQUARE_FONT_PATH}
};

enum FontSize {
    SMALL = 10,
    MEDIUM = 20,
    LARGE = 35,
    TITLE = 50
}

#[derive(Default)]
pub struct StartMenuState {
    font: Option<FontHandle>
}

impl SimpleState for StartMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.initialize_font(world);
        self.initialize_start_menu(world);
        self.initialize_camera(world);
    }
}

impl StartMenuState {
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

        world
            .create_entity()
            .with(title_transform)
            .with(title_text)
            .build();

        world
            .create_entity()
            .with(continue_transform)
            .with(continue_text)
            .build();
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

impl BaseState for StartMenuState {}
