// APP DIRECTORY
pub const APP_DIR: &str = ".";

// RESOURCE PATHS
pub const RESOURCES_PATH: &str = const_concat!(APP_DIR, "/", "resources");
pub const ASSETS_PATH: &str = const_concat!(RESOURCES_PATH, "/", "assets");
pub const CONFIG_PATH: &str = const_concat!(RESOURCES_PATH, "/", "config");

// ASSET PATHS
pub const TEXTURES_DIR: &str = "textures";
pub const FONTS_DIR: &str = "fonts";
pub const UI_LAYOUTS: &str = "ui_layouts";

// FONT PATHS
pub const SQUARE_FONT_PATH: &str = const_concat!(FONTS_DIR, "/", "square.ttf");

// CONFIG FILE PATHS
pub const DISPLAY_CONFIG_PATH: &str = const_concat!(CONFIG_PATH, "/", "display_config.ron");
pub const INPUT_BINDINGS_PATH: &str = const_concat!(CONFIG_PATH, "/", "input_bindings.ron");

// WINDOW CONSTANTS
pub const DEFAULT_WINDOW_DIMENSION_WIDTH: usize = 1000;
pub const DEFAULT_WINDOW_DIMENSION_HEIGHT: usize = 1000;

// ARENA CONSTANTS
pub const DEFAULT_ARENA_WIDTH: f32 = 1000.0;
pub const DEFAULT_ARENA_HEIGHT: f32 = 1000.0;

// COIN CONSTANTS
pub const COIN_PATH: &str = const_concat!(TEXTURES_DIR, "/", "coin");
pub const COIN_SPRITE_SHEET_PATH: &str = const_concat!(COIN_PATH, "/", "coin_sprite_sheet.png");
pub const COIN_RON_PATH: &str = const_concat!(COIN_PATH, "/", "coin_sprite_sheet.ron");
pub const COIN_TIME_PER_FRAME: f32 = 0.15;
pub const COIN_SPRITES_AMOUNT: usize = 10; // TODO: Find better way like iterating over sprite sheet once loaded

// PENGUIN CONSTANTS
pub const PENGUIN_PATH: &str = const_concat!(TEXTURES_DIR, "/", "penguin");
pub const PENGUIN_SPRITE_SHEET_PATH: &str = const_concat!(PENGUIN_PATH, "/", "penguin_sprite_sheet.png");
pub const PENGUIN_RON_PATH: &str = const_concat!(PENGUIN_PATH, "/", "penguin_sprite_sheet.ron");

// PLAYER CONSTANTS
pub const PLAYER_PATH: &str = const_concat!(TEXTURES_DIR, "/", "player");
pub const PLAYER_SPRITE_SHEET_PATH: &str = const_concat!(PLAYER_PATH, "/", "player_sprite_sheet.png");
pub const PLAYER_RON_PATH: &str = const_concat!(PLAYER_PATH, "/", "player_sprite_sheet.ron");

// MOVEMENT CONSTANTS
pub const DEFAULT_FRICTION: f32 = 2.0;

// MENU CONSTANTS
pub const LOSE_MENU_RON_PATH: &str = const_concat!(UI_LAYOUTS, "/", "lose_menu.ron");
pub const MAIN_MENU_RON_PATH: &str = const_concat!(UI_LAYOUTS, "/", "main_menu.ron");
pub const PAUSED_MENU_RON_PATH: &str = const_concat!(UI_LAYOUTS, "/", "paused_menu.ron");
pub const TITLE_MENU_RON_PATH: &str = const_concat!(UI_LAYOUTS, "/", "title_menu.ron");
pub const WIN_MENU_RON_PATH: &str = const_concat!(UI_LAYOUTS, "/", "win_menu.ron");

// NEURAL NETWORK CONSTANTS
pub const DEFAULT_NUM_OF_WEIGHTS: usize = 2;
pub const DEFAULT_LEARNING_RATE: f32 = 1.5;
