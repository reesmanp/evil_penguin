use amethyst::{
    assets::{Handle, ProgressCounter},
    core::{ArcThreadPool, Transform, math::base::Vector3},
    prelude::*,
    input::{VirtualKeyCode, is_key_down},
    ecs::{Dispatcher, DispatcherBuilder, Join},
    renderer::{SpriteSheet, SpriteRender}
};

use crate::{
    components::{
        entities::{
            CoinComponent,
            PenguinComponent,
            PlayerComponent
        },
        core::{
            EndConditionComponent,
            MovementComponent
        }
    },
    ml::neural_network::NeuralNetwork,
    states::{
        BaseState,
        LoadingState,
        NextLoadingState,
        PausedState
    },
    systems::{
        coins::{
            CoinCollectionSystem,
            CoinRotationSystem
        },
        end::{
            EndConditionSystem
        },
        movement::{
            EasyPenguinMovementSystem,
            Difficulty,
            HardPenguinMovementSystem,
            MediumPenguinMovementSystem,
            PlayerMovementSystem
        }
    },
    util::{
        constants::{
            DEFAULT_WINDOW_DIMENSION_WIDTH,
            DEFAULT_WINDOW_DIMENSION_HEIGHT,
            DEFAULT_ARENA_WIDTH,
            DEFAULT_ARENA_HEIGHT,
            COIN_SPRITE_SHEET_PATH,
            COIN_RON_PATH,
            COIN_SPRITES_AMOUNT,
            COIN_TIME_PER_FRAME,
            PENGUIN_SPRITE_SHEET_PATH,
            PENGUIN_RON_PATH,
            PLAYER_SPRITE_SHEET_PATH,
            PLAYER_RON_PATH,
            DEFAULT_FRICTION,
            DEFAULT_LEARNING_RATE
        },
        types::SpriteSheetLoadingData
    }
};
use std::collections::HashMap;

/// Run State
///
/// State in charge of the main game
/// Initializes -> Player, Penguin, Coins
pub struct RunState<'a, 'b> {
    coins: usize,
    coins_per_row: usize,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    progress_counter: ProgressCounter,
    coin_texture_handle: Option<Handle<SpriteSheet>>,
    penguin_texture_handle: Option<Handle<SpriteSheet>>,
    player_texture_handle: Option<Handle<SpriteSheet>>,
    difficulty: Difficulty
}

impl<'a, 'b> SimpleState for RunState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Initialize Dispatcher
        self.initialize_dispatcher(world);
        self.initialize_coins(world);
        self.initialize_penguin(world);
        self.initialize_player(world);
        self.initialize_camera(world);
        self.initialize_end_condition(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(e) = &event {
            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(LoadingState::new(NextLoadingState::Paused)));
            }
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            if let Some(dispatcher) = self.dispatcher.as_mut() {
                dispatcher.dispatch(&data.world);
            }

            let end_condition_storage = &data.world.read_storage::<EndConditionComponent>();
            if let Some(end_condition) = (end_condition_storage).join().next() {
                if let Some(is_win) = end_condition.is_win {
                    return Trans::Switch(Box::new(LoadingState::new(NextLoadingState::EndMenu(is_win))))
                }
            }
        }

        Trans::None
    }
}

impl<'a, 'b> RunState<'a, 'b> {
    pub fn new(coins: usize, coins_per_row: usize) -> Self {
        Self {
            coins,
            coins_per_row,
            dispatcher: None,
            progress_counter: ProgressCounter::new(),
            coin_texture_handle: None,
            penguin_texture_handle: None,
            player_texture_handle: None,
            difficulty: Difficulty::Easy
        }
    }

    pub fn new_from_dimensions(width: usize, height: usize) -> Self {
        Self::new((2 * width / 100) * (2 * height / 100), 2 * width / 100)
    }

    fn initialize_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(PlayerMovementSystem, "player_movement_system", &[]);

        match self.difficulty {
            Difficulty::Easy => dispatcher_builder.add(EasyPenguinMovementSystem, "penguin_movement_system", &["player_movement_system"]),
            Difficulty::Medium => dispatcher_builder.add(MediumPenguinMovementSystem, "penguin_movement_system", &["player_movement_system"]),
            Difficulty::Hard => {
                let mut hard_penguin_movement_system = HardPenguinMovementSystem::default();
                hard_penguin_movement_system.coin_amount = self.coins;
                dispatcher_builder.add(hard_penguin_movement_system, "penguin_movement_system", &["player_movement_system"])
            }
        }

        dispatcher_builder.add(CoinRotationSystem, "coin_rotation_system", &[]);
        dispatcher_builder.add(CoinCollectionSystem, "coin_collection_system", &["player_movement_system"]);
        dispatcher_builder.add(EndConditionSystem, "end_condition_system", &["penguin_movement_system", "coin_collection_system"]);

        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }

    fn initialize_player(&self, world: &mut World) {
        let sprite_render = SpriteRender {
            sprite_sheet: self.player_texture_handle.as_ref().unwrap().clone(),
            sprite_number: 0
        };
        let mut transform = Transform::default();
        let xyz_position = Vector3::new(30.0, 30.0, 0.0);
        transform.set_scale(Vector3::new(0.125, 0.125, 1.0));
        transform.set_translation_xyz(xyz_position.x, xyz_position.y, xyz_position.z);

        world
            .create_entity()
            .with(transform)
            .with(PlayerComponent::default())
            .with(MovementComponent::new(Vector3::new(0.0, 0.0, 0.0), 500.0, DEFAULT_FRICTION))
            .with(sprite_render)
            .build();
    }

    fn initialize_coins(&self, world: &mut World) {
        let sprite_render = SpriteRender {
            sprite_sheet: self.coin_texture_handle.as_ref().unwrap().clone(),
            sprite_number: 0
        };

        for i in 0..self.coins {
            let row = i / self.coins_per_row;
            let column = i % self.coins_per_row;
            let mut transform = Transform::default();
            transform.set_translation_xyz(50.0 * column as f32 + 25.0, 50.0 * row as f32 + 25.0, 0.0);
            transform.set_scale(Vector3::new(0.33, 0.33, 1.0));

            let mut coin = CoinComponent::default();
            coin.id = i;
            coin.total_amount = self.coins;
            coin.frames = COIN_SPRITES_AMOUNT;
            coin.time_per_frame = COIN_TIME_PER_FRAME;

            world
                .create_entity()
                .with(transform)
                .with(coin)
                .with(sprite_render.clone())
                .build();
        }
    }

    fn initialize_penguin(&self, world: &mut World) {
        let sprite_render = SpriteRender {
            sprite_sheet: self.penguin_texture_handle.as_ref().unwrap().clone(),
            sprite_number: 0
        };
        let mut transform = Transform::default();
        let xyz_position = Vector3::new(DEFAULT_ARENA_WIDTH / 2.0, DEFAULT_ARENA_HEIGHT / 2.0, 0.0);
        transform.set_scale(Vector3::new(0.33, 0.33, 1.0));
        transform.set_translation_xyz(xyz_position.x, xyz_position.y, xyz_position.z);

        world
            .create_entity()
            .with(transform)
            .with(PenguinComponent::default())
            .with(MovementComponent::new(Vector3::new(0.0, 0.0, 0.0), 500.0, DEFAULT_FRICTION)) // TODO: make the start acceleration and max speed variable based off of difficulty
            .with(sprite_render)
            .build();
    }

    fn initialize_end_condition(&self, world: &mut World) {
        world
            .create_entity()
            .with(EndConditionComponent::default())
            .build();
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }
}

impl<'a, 'b> Default for RunState<'a, 'b> {
    fn default() -> Self {
        Self::new_from_dimensions(
            DEFAULT_WINDOW_DIMENSION_WIDTH,
            DEFAULT_WINDOW_DIMENSION_HEIGHT
        )
    }
}

impl<'a, 'b> BaseState for RunState<'a, 'b> {
    fn get_dependent_spritesheets() -> Vec<SpriteSheetLoadingData<'static>> {
        vec![
            ("coin", COIN_SPRITE_SHEET_PATH, COIN_RON_PATH),
            ("penguin", PENGUIN_SPRITE_SHEET_PATH, PENGUIN_RON_PATH),
            ("player", PLAYER_SPRITE_SHEET_PATH, PLAYER_RON_PATH)
        ]
    }

    fn set_dependent_spritesheet_handles(&mut self, handle_map: &mut HashMap<String, Handle<SpriteSheet>>) {
        self.coin_texture_handle = handle_map.remove("coin").take();
        self.penguin_texture_handle = handle_map.remove("penguin").take();
        self.player_texture_handle = handle_map.remove("player").take();
    }
}
