use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

#[derive(Resource, Debug, Default)]
pub struct CatAssets {
    pub pelt_color: Handle<Image>,
    pub lineart: Handle<Image>,
}

pub struct AssetLoaderPlugin;

// TODO: Have this set up of "loading asset" state
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CatAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut cat_assets: ResMut<CatAssets>, asset_server: Res<AssetServer>) {
    *cat_assets = CatAssets {
        pelt_color: asset_server.load("classiccolours.png"),
        lineart: asset_server.load("lineart.png"),
    }
}