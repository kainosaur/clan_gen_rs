use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::ConfigureLoadingState;
use cats::*;
use cats::sprites::*;
use assets::*;

mod assets;
mod cats;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(AssetLoaderPlugin)
        .add_systems(PostStartup, create_random_cat)
        .add_systems(Update, refresh_cat_layers)
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}





#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,)]
pub enum GameState {
    #[default]
    AssetLoading,
    Menu,
    NameClan,
    PickCats,
    YourClan,
}