use bevy::prelude::*;
use crate::CatAssets;

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

pub const CAT_COLORS: &[CatColor] = &[
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

// This is for matching how the sprite sheet looks.
pub const COLOR_CATEGORIES: &[&[CatColor]] = &[
    &[
        CatColor::White,
        CatColor::PaleGrey,
        CatColor::Silver,
        CatColor::Grey,
        CatColor::DarkGrey,
        CatColor::Ghost,
        CatColor::Black,
    ],
    &[
        CatColor::Cream,
        CatColor::PaleGinger,
        CatColor::Golden,
        CatColor::Ginger,
        CatColor::DarkGinger,
        CatColor::Sienna,
    ],
    &[
        CatColor::LightBrown,
        CatColor::Lilac,
        CatColor::Brown,
        CatColor::GoldenBrown,
        CatColor::DarkBrown,
        CatColor::Chocolate,
    ],
];

#[derive(Debug, PartialEq, Eq, Hash, Component, Clone, Copy)]
pub enum CatStatus {
    NewBorn,
    Kitten,
    Apprentice,
    Warrior,
    Elder,
}

pub const CAT_STATUSES: &[CatStatus] = &[
    CatStatus::NewBorn,
    CatStatus::Kitten,
    CatStatus::Apprentice,
    CatStatus::Warrior,
    CatStatus::Elder,
];
// TODO: Remove this
pub const STATUS_CATEGORIES: &[CatStatus] = &[
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
    Hazel,
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
    PaleYellow,
    Gold,
    GreenYellow,
}

pub const CAT_EYE_COLORS: &[CatEyeColor] = &[
    CatEyeColor::Yellow,
    CatEyeColor::Amber,
    CatEyeColor::Hazel,
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
    CatEyeColor::PaleYellow,
    CatEyeColor::Gold,
    CatEyeColor::GreenYellow,
];

// This is for how it is aligned on the sprite sheet
pub const EYE_CATEGORIES: &[&[CatEyeColor]] = &[
    &[
        CatEyeColor::Yellow,
        CatEyeColor::Amber,
        CatEyeColor::Hazel,
        CatEyeColor::PaleGreen,
        CatEyeColor::Green,
        CatEyeColor::Blue,
        CatEyeColor::DarkBlue,
        CatEyeColor::Grey,
        CatEyeColor::Cyan,
        CatEyeColor::Emerald,
        CatEyeColor::HeatherBlue,
        CatEyeColor::SunLitice,
    ],
    &[
        CatEyeColor::Copper,
        CatEyeColor::Sage,
        CatEyeColor::Cobalt,
        CatEyeColor::PaleBlue,
        CatEyeColor::Bronze,
        CatEyeColor::Silver,
        CatEyeColor::PaleYellow,
        CatEyeColor::GreenYellow,
    ],
];

pub fn create_cat_sprites_system(
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    cat_assets: Res<CatAssets>,
    cat_status: CatStatus,
    cat_color: CatColor,
    cat_eye_color: CatEyeColor,
) -> Option<(SpriteSheetBundle, SpriteSheetBundle, SpriteSheetBundle)> {
    
    if let Some((lineart_index, status_row)) = load_lineart(cat_status) {
        let color_layout = TextureAtlasLayout::from_grid(Vec2::new(50.0, 50.0), 21, 21, None, None);
        let texture_atlas_color_layout = texture_atlas_layouts.add(color_layout);
        let color_index = load_color(cat_color, status_row).expect("Color not found");
        let lineart_layout = TextureAtlasLayout::from_grid(Vec2::new(50.0, 50.0), 3, 7, None, None);
        let texture_atlas_lineart_layout = texture_atlas_layouts.add(lineart_layout);

        let eye_layout =  TextureAtlasLayout::from_grid(Vec2::new(50.0, 50.0), 36, 14, None, None);
        let eye_index = load_eye(cat_eye_color, status_row).expect("cat eye not found");
        let texture_atlas_eye_layout = texture_atlas_layouts.add(eye_layout);

        let pelt_atlas = TextureAtlas {
            layout: texture_atlas_color_layout,
            index: color_index
        };
    
        let lineart_atlas = TextureAtlas {
            layout: texture_atlas_lineart_layout,
            index: lineart_index
        };

        let eye_atlas = TextureAtlas {
            layout: texture_atlas_eye_layout,
            index: eye_index,
        };
    
        let pelt_bundle = SpriteSheetBundle {
            texture: cat_assets.pelt_color.clone(),
            atlas: pelt_atlas,
            visibility: Visibility::Hidden,
            ..Default::default()
        };

        let lineart_bundle = SpriteSheetBundle {
            texture: cat_assets.lineart.clone(),
            atlas: lineart_atlas,
            visibility: Visibility::Hidden,
            ..Default::default()
        };

        let eye_bundle = SpriteSheetBundle {
            texture: cat_assets.eye_color.clone(),
            atlas: eye_atlas,
            visibility: Visibility::Hidden,
            ..Default::default()
        };

        return Some((pelt_bundle, lineart_bundle, eye_bundle))
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


fn load_eye(color_name: CatEyeColor, lineart_wanted: usize) -> Option<usize> {
    for (row, colors) in EYE_CATEGORIES.iter().enumerate() {
        for (col, color) in colors.iter().enumerate() {
            if color_name == *color {
                let y = row * 7 + lineart_wanted;
                let x = col * 3;
                let index = get_sprite_index(x, y, 36);
                println!("{} (position Y:{}), {} (position X:{}) is the eye color {:?}. Index: {}", row, y, col, x, color, index);
                return Some(index); // return the index
            }
        }
    }
    None // return None if the color is not found
}
