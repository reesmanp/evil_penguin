use amethyst::{
    assets::{Handle, Loader, AssetStorage, ProgressCounter},
    core::{ArcThreadPool, Transform, math::base::Vector3},
    prelude::*,
    input::{VirtualKeyCode, is_key_down},
    ecs::{Dispatcher, DispatcherBuilder, Join},
    renderer::{SpriteSheet, SpriteRender, Texture, ImageFormat, SpriteSheetFormat, Camera}
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
    states::GamePausedState,
    systems::{
        coins::{
            CoinCollectionSystem,
            CoinRotationSystem
        },
        end::{
            EndConditionSystem
        },
        movement::{
            PenguinMovementSystem,
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
            PENGUIN_SPRITE_SHEET_PATH,
            PENGUIN_RON_PATH
        },
        types::TextureAndRonTuple
    }
};


pub struct GameRunState<'a, 'b> {
    coins: usize,
    coins_per_row: usize,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    progress_counter: ProgressCounter
}

impl<'a, 'b> SimpleState for GameRunState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load Textures
        let coin_texture_handle = self.load_sprite_sheet(world, (COIN_SPRITE_SHEET_PATH, COIN_RON_PATH));
        let penguin_texture_handle = self.load_sprite_sheet(world, (PENGUIN_SPRITE_SHEET_PATH, PENGUIN_RON_PATH));
        let player_texture_handle = self.load_sprite_sheet(world, (PENGUIN_SPRITE_SHEET_PATH, PENGUIN_RON_PATH));

        // Initialize State Items
        self.initialize_dispatcher(world);
        self.initialize_coins(world, coin_texture_handle);
        self.initialize_penguin(world, penguin_texture_handle);
        self.initialize_player(world, player_texture_handle);
        self.initialize_camera(world);
        self.initialize_end_condition(world);
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(e) = &event {
            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(GamePausedState));
            }
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        let end_condition_storage = &data.world.read_storage::<EndConditionComponent>();
        let end_condition = (end_condition_storage).join().next().unwrap();
        if let Some(is_win) = end_condition.is_win {
            // TODO: Handle win or lose
            Trans::Quit
        } else {
            Trans::None
        }
    }
}

impl<'a, 'b> GameRunState<'a, 'b> {
    pub fn new(coins: usize, coins_per_row: usize) -> Self {
        Self {
            coins,
            coins_per_row,
            dispatcher: None,
            progress_counter: ProgressCounter::new()
        }
    }

    pub fn new_from_dimensions(width: usize, height: usize) -> Self {
        Self::new((2 * width / 100) * (2 * height / 100), 2 * width / 100)
    }

    fn initialize_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(CoinRotationSystem, "coin_rotation_system", &[]);
        dispatcher_builder.add(PlayerMovementSystem, "player_movement_system", &[]);
        dispatcher_builder.add(PenguinMovementSystem, "penguin_movement_system", &["player_movement_system"]);
        dispatcher_builder.add(CoinCollectionSystem, "coin_collection_system", &["player_movement_system"]);
        dispatcher_builder.add(EndConditionSystem, "end_condition_system", &["penguin_movement_system", "coin_collection_system"]);

        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }

    fn load_sprite_sheet(
        &mut self,
        world: &mut World,
        (sprite_sheet_path, ron_path): TextureAndRonTuple
    ) -> Handle<SpriteSheet>
    {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                sprite_sheet_path,
                ImageFormat::default(),
                &mut self.progress_counter,
                &texture_storage
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            ron_path,
            SpriteSheetFormat(texture_handle),
            &mut self.progress_counter,
            &sprite_sheet_store
        )
    }

    fn initialize_player(&self, world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0
        };
        let mut transform = Transform::default();
        let xyz_position = Vector3::new(30.0, 30.0, 0.0);
        transform.set_scale(Vector3::new(0.33, 0.33, 1.0));
        transform.set_translation_xyz(xyz_position.x, xyz_position.y, xyz_position.z);

        world
            .create_entity()
            .with(transform)
            .with(PlayerComponent::default())
            .with(MovementComponent::new(Vector3::new(0.0, 0.0, 0.0), 500.0, 2.0))
            .with(sprite_render)
            .build();
    }

    fn initialize_coins(&self, world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0
        };

        for i in 0..self.coins {
            let row = i / self.coins_per_row;
            let column = i % self.coins_per_row;
            let mut transform = Transform::default();
            transform.set_translation_xyz(50.0 * column as f32 + 25.0, 50.0 * row as f32 + 25.0, 0.0);
            transform.set_scale(Vector3::new(0.33, 0.33, 1.0));

            let mut coin = CoinComponent::default();
            coin.frames = COIN_SPRITES_AMOUNT;

            // TODO: detect row and column for coin <-- why?
            world
                .create_entity()
                .with(transform)
                .with(coin)
                .with(sprite_render.clone())
                .build();
        }
    }

    fn initialize_penguin(&self, world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
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
            .with(MovementComponent::new(Vector3::new(0.0, 0.0, 0.0), 500.0, 2.0)) // TODO: make the start acceleration and max speed variable based off of difficulty
            .with(sprite_render)
            .build();
    }

    fn initialize_camera(&self, world: &mut World) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(DEFAULT_ARENA_WIDTH / 2.0, DEFAULT_ARENA_HEIGHT / 2.0, 5.0);

        world
            .create_entity()
            .with(transform)
            .with(Camera::standard_2d(DEFAULT_ARENA_WIDTH, DEFAULT_ARENA_HEIGHT))
            .build();
    }

    fn initialize_end_condition(&self, world: &mut World) {
        world
            .create_entity()
            .with(EndConditionComponent::default())
            .build();
    }
}

impl<'a, 'b> Default for GameRunState<'a, 'b> {
    fn default() -> Self {
        Self::new_from_dimensions(
            DEFAULT_WINDOW_DIMENSION_WIDTH,
            DEFAULT_WINDOW_DIMENSION_HEIGHT
        )
    }
}
