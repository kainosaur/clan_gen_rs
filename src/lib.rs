use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,)]
pub enum GameState {
    #[default]
    AssetLoading,
    Menu,
    NameClan,
    PickCats,
    YourClan,
}