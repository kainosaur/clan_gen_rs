use bevy::prelude::*;
use crate::cats::{CatColor, CatStatus};


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



pub const STATUS_CATEGORIES: &[CatStatus] = &[
    CatStatus::NewBorn,
    CatStatus::Kitten,
    CatStatus::Apprentice,
    CatStatus::Warrior,
    CatStatus::Elder,
];

// 
