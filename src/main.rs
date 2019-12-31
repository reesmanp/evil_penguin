use amethyst::{
    assets::Processor,
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::{Application, GameDataBuilder},
    renderer::{SpriteSheet, RenderingSystem, types::DefaultBackend, RenderingBundle, RenderToWindow, RenderFlat2D},
    utils::application_root_dir
};

mod components;
mod states;
mod systems;
mod util;

use states::GameRunState;

fn main() -> amethyst::Result<()> {
    // Logging
    amethyst::start_logger(Default::default());

    // Directory Paths
    let app_root = application_root_dir()?;
    let resources_dir = app_root.join("resources");
    let assets_path = resources_dir.join("assets");

    // Config Paths
    let display_config_path = resources_dir.join("display_config.ron");
    let input_bindings_path = resources_dir.join("input_bindings.ron");

    // Create Bundles
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(input_bindings_path)?;
    let transform_bundle = TransformBundle::new();
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderToWindow::from_config_path(display_config_path)
                         .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default());

    // Create Game Data
    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with_bundle(transform_bundle)?
        .with_bundle(rendering_bundle)?;

    // Create Starting State
    let game_run_state = GameRunState::default();

    // Create Game
    let mut game = Application::new(assets_path, game_run_state, game_data)?;
    game.run();

    Ok(())
}
