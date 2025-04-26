use bevy::prelude::*;

use crate::game_state::GameState;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartMenu), setup)
            .add_systems(Update, menu.run_if(in_state(GameState::StartMenu)))
            .add_systems(OnExit(GameState::StartMenu), teardown);
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Resource)]
struct MenuData {
    button_entity: Entity,
}

fn setup(mut commands: Commands) {
    let button_entity = commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(50.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Play"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        })
        .id();

    commands.insert_resource(MenuData { button_entity });
}

fn menu(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn teardown(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn();
}
