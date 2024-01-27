#![allow(clippy::type_complexity)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{
    prelude::*,
    window::{close_on_esc, EnabledButtons},
};
use bevy_embedded_assets::EmbeddedAssetPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.8, 0.6, 0.5)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                position: WindowPosition::Automatic,
                title: "<3".into(),
                resolution: (480., 480.).into(),
                resizable: false,
                enabled_buttons: EnabledButtons {
                    maximize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .add_systems(Update, close_on_esc)
        .run();
}

#[derive(Component)]
struct YesButton;

#[derive(Component)]
struct Buttons;

#[derive(Component)]
struct Heading;

const NORMAL_BUTTON: Color = Color::rgb(0.75, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                margin: UiRect::top(Val::Percent(20.)),
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style {
                        width: Val::Px(400.),
                        height: Val::Px(100.),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        "Will you be my valentine?",
                        TextStyle {
                            font_size: 30.,
                            color: Color::BLACK,
                            ..default()
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                },
                Heading,
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            },
            Buttons,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    YesButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Yes",
                        TextStyle {
                            font_size: 20.0,
                            ..default()
                        },
                    ));
                });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "No",
                        TextStyle {
                            font_size: 20.0,
                            ..default()
                        },
                    ));
                });
        });
}

fn button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut Visibility,
            &Children,
            Has<YesButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text, Without<Heading>>,
    mut heading_query: Query<&mut Text, With<Heading>>,
    buttons_query: Query<Entity, With<Buttons>>,
) {
    for (interaction, mut color, mut visibility, children, yes) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if !yes {
                    *visibility = Visibility::Hidden
                } else {
                    let us = asset_server.load("embedded://background.png");

                    commands.spawn(SpriteBundle {
                        texture: us,
                        ..default()
                    });

                    let mut heading = heading_query.single_mut();
                    heading.sections[0].value = "YAYYYYY".into();
                    commands.entity(buttons_query.single()).despawn_recursive();
                }
            }
            Interaction::Hovered => {
                text.sections[0].value = if yes { "PLEASE".into() } else { ":(".into() };
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = if yes { "Yes".into() } else { "No".into() };
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
