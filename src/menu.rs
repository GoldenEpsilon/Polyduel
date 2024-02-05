use crate::*;

use bevy::prelude::*;


#[derive(Component)]
pub struct MenuButton {
    button_type: ButtonType
}

pub enum ButtonType {
    Online,
    Offline
}

#[derive(Resource)]
pub struct MenuData {
    button_entity: Entity,
}

pub fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }
        )
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: BackgroundColor(Color::RED),
                    ..default()
                }, MenuButton { button_type: ButtonType::Offline }))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        }).id();
    commands.insert_resource(MenuData { button_entity });
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &MenuButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameState>>,
    mut network_state: ResMut<NextState<NetworkState>>
) {
    for (interaction, mut color, mut border_color, children, button) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::BLUE);
                border_color.0 = Color::RED;
                match button.button_type {
                    ButtonType::Offline => {
                        network_state.set(NetworkState::Offline);
                        next_state.set(GameState::Gameplay);
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::YELLOW);
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = BackgroundColor(Color::GREEN);
                border_color.0 = Color::BLACK;
            }
        }
        match button.button_type {
            ButtonType::Offline => {
                text.sections[0].value = "Offline".to_string();
            }
            _ => {
                text.sections[0].value = "Button".to_string();
            }
        }
    }
}

pub fn menu_cleanup(
    mut commands: Commands,
    menu_data: Res<MenuData>
) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}