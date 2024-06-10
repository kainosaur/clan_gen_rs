use bevy::{prelude::*, render::view::visibility};
use names::*;
use rand::{seq::SliceRandom, thread_rng};
use sprites::*;
use super::GameState;

use crate::CatAssets;

pub mod names;
pub mod sprites;

pub struct CatsPlugin;

impl Plugin for CatsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), create_random_cat)
            .add_systems(Update,
             (refresh_all_layers,)
             .run_if(in_state(GameState::Menu))
            );
    }
}

#[derive(Bundle)]
pub struct CatBundle {
    name: Prefix,
    suffix: Suffix,
    pelt_color: CatColor,
    status: CatStatus,
    eye_color: CatEyeColor,
    visibility: InheritedVisibility,
    global_transform: GlobalTransform,
    transform: Transform,
}

// impl Default for CatBundle {
//     fn default() -> Self {
//         Self {
//             name: Prefix::Ant,
//             suffix: random_suffix(),
//             pelt_color: CatColor::Cream,
//             status: CatStatus::Warrior,
//             eye_color: CatEyeColor::Blue,
//             pelt_sprite_bundle: SpriteBundle {
//                 texture: 
//             }
//         }
//     }
// }

pub fn random_cat_color() -> CatColor {
    let mut rng = thread_rng();
    *CAT_COLORS.choose(&mut rng).unwrap()
}

pub fn random_cat_eye_color() -> CatEyeColor {
    let mut rng = thread_rng();
    *CAT_EYE_COLORS.choose(&mut rng).unwrap()
}

pub fn random_cat_status() -> CatStatus {
    let mut rng = thread_rng();
    *CAT_STATUSES.choose(&mut rng).unwrap()
}

#[derive(Component)]
pub struct Cat;

#[derive(Component)]
pub struct Color;

#[derive(Component)]
pub struct Lineart;

#[derive(Component)]
pub struct Eye;

pub fn create_cat(
    mut commands: Commands,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    cat_assets: Res<CatAssets>,
    cat_eye_color: CatEyeColor,
    cat_color: CatColor,
    cat_status: CatStatus,
    name: Prefix,
    suffix: Suffix,
) {

    if let Some((lineart_bundle, pelt_bundle, eye_bundle)) = create_cat_sprites_system(
        texture_atlas_layouts,
        cat_assets,
        cat_status,
        cat_color,
        cat_eye_color,
    ) {
        commands.spawn((Cat, CatBundle {
            name: name,
            suffix: suffix,
            pelt_color: cat_color,
            eye_color: cat_eye_color,
            status: cat_status,
            visibility: InheritedVisibility::VISIBLE,
            global_transform: GlobalTransform::from_scale(Vec3::splat(32.0)),
            transform: Transform::from_scale(Vec3::splat(12.0)),
        })).with_children(|parent| {
            parent.spawn((Color, pelt_bundle ));
            parent.spawn((Lineart, lineart_bundle));
            parent.spawn((Eye, eye_bundle));
        });
    } else {

    }
}


pub fn create_random_cat(
    commands: Commands,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    cat_assets: Res<CatAssets>,
) {
    let color = random_cat_color();
    let status = random_cat_status();
    let name = random_prefix();
    let suffix = random_suffix();
    let eye_color = random_cat_eye_color();
    create_cat(
        commands, 
        texture_atlas_layouts, 
        cat_assets, 
        eye_color,
        color, 
        status, 
        name, 
        suffix,
    )
}

pub fn greet_cats(query: Query<(&Prefix, &Suffix), With<Cat>>) {
    for (name, suffix) in &query {
        println!("*meows a greeting to {}{}*", name, suffix);
    }
}

pub fn refresh_all_layers(
    mut param_set: ParamSet<(
        Query<&mut Visibility, With<Lineart>>,
        Query<&mut Visibility, With<Color>>,
        Query<&mut Visibility, With<Eye>>,
    )>,
) {
    refresh_lineart_layers(param_set.p0());
    refresh_color_layers(param_set.p1());
    refresh_eye_layers(param_set.p2());
}


fn refresh_lineart_layers(mut query: Query<&mut Visibility, With<Lineart>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
        *visibility = Visibility::Inherited;
    }
}

fn refresh_color_layers(mut query: Query<&mut Visibility, With<Color>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
        *visibility = Visibility::Inherited;
    }
}

fn refresh_eye_layers(mut query: Query<&mut Visibility, With<Eye>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
        *visibility = Visibility::Inherited;
    }
}