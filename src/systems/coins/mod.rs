#[path = "coin_collection.system.rs"]
mod coin_collection_system;
#[path = "coin_rotation.system.rs"]
mod coin_rotation_system;

pub use self::{
    coin_collection_system::CoinCollectionSystem,
    coin_rotation_system::CoinRotationSystem
};
