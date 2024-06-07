use bevy::{prelude::*, render::view::visibility};
use names::*;
use rand::{seq::SliceRandom, thread_rng};
use sprites::*;

use crate::CatAssets;

pub mod names;
pub mod sprites;

pub struct CatsPlugin;

#[derive(Debug, PartialEq, Eq, Hash, Component, Clone, Copy)]
pub enum CatColor {
    White,
    PaleGrey,
    Silver,
    Grey,
    DarkGrey,
    Ghost,
    Black,
    Cream,
    PaleGinger,
    Golden,
    Ginger,
    DarkGinger,
    Sienna,
    LightBrown,
    Lilac,
    Brown,
    GoldenBrown,
    DarkBrown,
    Chocolate,
}

const CAT_COLORS: &[CatColor] = &[
    CatColor::White,
    CatColor::PaleGrey,
    CatColor::Silver,
    CatColor::Grey,
    CatColor::DarkGrey,
    CatColor::Ghost,
    CatColor::Black,
    CatColor::Cream,
    CatColor::PaleGinger,
    CatColor::Golden,
    CatColor::Ginger,
    CatColor::DarkGinger,
    CatColor::Sienna,
    CatColor::LightBrown,
    CatColor::Lilac,
    CatColor::Brown,
    CatColor::GoldenBrown,
    CatColor::DarkBrown,
    CatColor::Chocolate,
];

#[derive(Debug, PartialEq, Eq, Hash, Component, Clone, Copy)]
pub enum CatStatus {
    NewBorn,
    Kitten,
    Apprentice,
    Warrior,
    Elder,
}

const CAT_STATUSES: &[CatStatus] = &[
    CatStatus::NewBorn,
    CatStatus::Kitten,
    CatStatus::Apprentice,
    CatStatus::Warrior,
    CatStatus::Elder,
];


#[derive(Debug, PartialEq, Eq, Hash, Component, Clone, Copy)]
pub enum CatEyeColor {
    Yellow,
    Amber,
    Hazle,
    PaleGreen,
    Green,
    Blue,
    DarkBlue,
    Grey,
    Cyan,
    Emerald,
    HeatherBlue,
    SunLitice,
    Copper,
    Sage,
    Cobalt,
    PaleBlue,
    Bronze,
    Silver,
}

const CAT_EYE_COLORS: &[CatEyeColor] = &[
    CatEyeColor::Yellow,
    CatEyeColor::Amber,
    CatEyeColor::Hazle,
    CatEyeColor::PaleGreen,
    CatEyeColor::Green,
    CatEyeColor::Blue,
    CatEyeColor::DarkBlue,
    CatEyeColor::Grey,
    CatEyeColor::Cyan,
    CatEyeColor::Emerald,
    CatEyeColor::HeatherBlue,
    CatEyeColor::SunLitice,
    CatEyeColor::Copper,
    CatEyeColor::Sage,
    CatEyeColor::Cobalt,
    CatEyeColor::PaleBlue,
    CatEyeColor::Bronze,
    CatEyeColor::Silver,
];

#[derive(Bundle)]
pub struct CatBundle {
    name: Prefix,
    suffix: Suffix,
    pelt_color: CatColor,
    status: CatStatus,
    // eye_color: CatEyeColor,
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

pub fn create_cat(
    mut commands: Commands,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    cat_assets: Res<CatAssets>,
    cat_color: CatColor,
    cat_status: CatStatus,
    name: Prefix,
    suffix: Suffix,
) {

    if let Some((lineart_bundle, pelt_bundle)) = sprites_system(
        texture_atlas_layouts,
        cat_assets,
        cat_status,
        cat_color,
    ) {
        commands.spawn((Cat, CatBundle {
            name: name,
            suffix: suffix,
            pelt_color: cat_color,
            status: cat_status,
            visibility: InheritedVisibility::VISIBLE,
            global_transform: GlobalTransform::from_scale(Vec3::splat(32.0)),
            transform: Transform::from_scale(Vec3::splat(12.0)),
        })).with_children(|parent| {
            parent.spawn((Color, pelt_bundle ));
            parent.spawn((Lineart, lineart_bundle));
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
    create_cat(
        commands, 
        texture_atlas_layouts, 
        cat_assets, 
        color, 
        status, 
        name, 
        suffix,
    )
}

pub fn sprites_system(
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    cat_assets: Res<CatAssets>,
    cat_status: CatStatus,
    cat_color: CatColor,
) -> Option<(SpriteSheetBundle, SpriteSheetBundle)> {
    
    if let Some((lineart_index, status_row)) = load_lineart(cat_status) {
        let color_layout = TextureAtlasLayout::from_grid(Vec2::new(50.0, 50.0), 21, 21, None, None);
        let texture_atlas_color_layout = texture_atlas_layouts.add(color_layout);
        let color_index = load_color(cat_color, status_row).expect("Color not found");
        let lineart_layout = TextureAtlasLayout::from_grid(Vec2::new(50.0, 50.0), 3, 7, None, None);
        let texture_atlas_lineart_layout = texture_atlas_layouts.add(lineart_layout);

        let pelt_atlas = TextureAtlas {
            layout: texture_atlas_color_layout,
            index: color_index
        };
    
        let lineart_atlas = TextureAtlas {
            layout: texture_atlas_lineart_layout,
            index: lineart_index
        };
    
        let pelt_bundle = SpriteSheetBundle {
            texture: cat_assets.pelt_color.clone(),
            atlas: pelt_atlas,
            ..Default::default()
        };

        let lineart_bundle = SpriteSheetBundle {
            texture: cat_assets.lineart.clone(),
            atlas: lineart_atlas,
            ..Default::default()
        };
        return Some((pelt_bundle, lineart_bundle))
    } else {
        println!("Lineart not found")
    }
    None
}

fn get_sprite_index(x: usize, y: usize, sprites_per_row:usize) -> usize{
    y * sprites_per_row + x
}

fn load_color(color_name: CatColor, lineart_wanted: usize) -> Option<usize> {
    for (row, colors) in COLOR_CATEGORIES.iter().enumerate() {
        for (col, color) in colors.iter().enumerate() {
            if color_name == *color {
                let y = row * 7 + lineart_wanted;
                let x = col * 3;
                let index = get_sprite_index(x, y, 21);
                println!("{} (position Y:{}), {} (position X:{}) is the color {:?}. Index: {}", row, y, col, x, color, index);
                return Some(index); // return the index
            }
        }
    }
    None // return None if the color is not found
}


fn load_lineart(lineart_wanted: CatStatus) -> Option<(usize, usize)> {
    for (row, status) in STATUS_CATEGORIES.iter().enumerate() {
        if lineart_wanted == *status {
            let y = row * 3;
            let index = y;
            println!("{} is pose {:?}. Index: {}", row, status, index);
            return Some((index, row))
        }
    }
    None
}



pub fn greet_cats(query: Query<(&Prefix, &Suffix), With<Cat>>) {
    for (name, suffix) in &query {
        println!("*meows a greeting to {}{}*", name, suffix);
    }
}

// fn inspect_cats(world: &World) {
//     println!("{:#?}", world.inspect_entity(Cat));
// }
pub fn refresh_cat_layers(query_lineart: Query<&mut Visibility, With<Lineart>>) {
    for visibility in &query_lineart {
        let visibility = &Visibility::Visible;
    }
}