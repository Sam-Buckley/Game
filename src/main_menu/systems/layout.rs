use std::vec;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

// System
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity: Entity = build_main_menu(&mut commands, &asset_server);
}

// System
pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Not a system
fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let mut main_menu_entity = commands.spawn((
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(30.0),
                row_gap: Val::Px(30.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: Color::srgba(0.15, 0.15, 0.15, 0.35).into(),
            ..Default::default()
        },
        MainMenu {},
    ));

    add_play_button(&asset_server, &mut main_menu_entity);
    add_quit_button(&asset_server, &mut main_menu_entity);
    main_menu_entity.id()
}

fn add_play_button(asset_server: &&Res<AssetServer>, parent: &mut EntityCommands) {
    parent.with_children(|parent| {
        // === Title ===

        // === Play Button ===
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: BUTTON_COLOR.into(),
                    ..Default::default()
                },
                PlayButton {},
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Play".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        }],
                        justify: JustifyText::Center,
                        ..Default::default()
                    },
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
        // === Quit Button ===
    });
}

fn add_quit_button(asset_server: &&Res<AssetServer>, parent: &mut EntityCommands) {
    parent.with_children(|parent| {
        // === Title ===

        // === Quit Button ===
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: BUTTON_COLOR.into(),
                    ..Default::default()
                },
                QuitButton {},
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Quit".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        }],
                        justify: JustifyText::Center,
                        ..Default::default()
                    },
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    });
}
