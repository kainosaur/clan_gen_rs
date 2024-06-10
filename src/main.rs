use bevy::{prelude::*, window::WindowResolution};
// use bevy_egui::{egui::{self, Pos2}, EguiContexts, EguiPlugin};
use assets::*;
use cats::{create_random_cat, Cat};

mod assets;
mod cats;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Startup, main_menu)
        .add_plugins(AssetLoaderPlugin)
        // .add_plugins(CatsPlugin)
        .add_systems(Update, 
            (button_system)
                .run_if(in_state(GameState::Menu))
        )
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

pub struct MainMenuPlugin;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct SelectedOption;

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

// //Looks for the random cat that was just generated specifically for title screen
// fn find_cat(query: Query<&mut Transform, With<Cat>>) {
//     for transform in &query {
//         *transform = Transform {
//             todo!()
//         }
//     }
// }

fn main_menu (mut commands: Commands,) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    let ui_container = NodeBundle{
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let menu_container = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        // background_color: Color::CRIMSON.into(),
        ..default()
    };

    let menu_screen_text = TextBundle::from_section(
        "Welcome to ClanGen",
        TextStyle {
            font_size: 80.0,
            ..default()
        },
    );

    let cat_container = NodeBundle {
        style: Style {
            width: Val::Px(250.0),
            height: Val::Px(250.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let continue_button = ButtonBundle {
        style: button_style.clone(),
        ..default()
    };
    let continue_button_text = TextBundle::from_section(
        "Continue Game",
        button_text_style.clone(),
    );

    let start_button = ButtonBundle {
        style: button_style.clone(),
        ..default()
    };

    // Spawns UI
    let ui_container_entity = commands.spawn(ui_container).id();
    let menu_container_entity = commands.spawn(menu_container).id();
    let menu_screen_text_container = commands.spawn(menu_screen_text).id();
    // Components of where the cat is held
    let cat_container_entity = commands.spawn(cat_container).id();
    // Components that make up the "Continue" button
    let continue_button_entity = commands.spawn(continue_button).id();
    let continue_button_text_entity = commands.spawn(continue_button_text).id();

    // Components that make up the "Start" button
    let start_button_entity = commands.spawn(start_button).id();

    // For parenting UI to container
    commands.entity(ui_container_entity).push_children(&[menu_container_entity]);
    // Ordering of the menu
    commands.entity(menu_container_entity).push_children(
        &[menu_screen_text_container, 
        cat_container_entity,
        continue_button_entity, 
        start_button_entity]
    );
    // Randomly generated cat on main menu parenting random cat
    // commands.entity(cat_container_entity).push_children(&[create_random_cat]);
    // "Continue" button parenting
    commands.entity(continue_button_entity).push_children(
        &[continue_button_text_entity]
    );
}



// Title: "Welcome to ClanGen"
// Cat: Random Cat
// *** START WITH ONE Button: Continue Game todo!(make a save file system TODO)
// Button: Start Game



#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,)]
pub enum GameState {
    #[default]
    AssetLoading,
    Menu,
    NameClan,
    PickCats,
    YourClan,
}