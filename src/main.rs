#[macro_use]
extern crate const_concat;

use amethyst::{
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::{Application, GameDataBuilder},
    renderer::{types::DefaultBackend, RenderingBundle, RenderToWindow, RenderFlat2D},
    ui::{RenderUi, UiBundle}
};

mod components;
mod states;
mod systems;
mod util;

use states::RunState;
use states::StartMenuState;
use util::constants::{
    ASSETS_PATH,
    DISPLAY_CONFIG_PATH,
    INPUT_BINDINGS_PATH
};

fn main() -> amethyst::Result<()> {
    // Logging
    amethyst::start_logger(Default::default());

    // Create Bundles
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(INPUT_BINDINGS_PATH)?;
    let transform_bundle = TransformBundle::new();
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderToWindow::from_config_path(DISPLAY_CONFIG_PATH)
                         .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default());
    let ui_bundle = UiBundle::<StringBindings>::new();

    // Create Game Data
    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with_bundle(transform_bundle)?
        .with_bundle(rendering_bundle)?
        .with_bundle(ui_bundle)?;

    // Create Starting State
    let run_state = RunState::default();
    let start_menu_state = StartMenuState::default();

    // Create Game
    let mut game = Application::new(ASSETS_PATH, start_menu_state, game_data)?;
    game.run();

    Ok(())
}
