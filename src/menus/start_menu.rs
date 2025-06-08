use bevy::{ecs::spawn::SpawnWith, prelude::*};

use crate::{
    game_states::GameState,
    menus::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
};

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartMenu), setup)
            .add_systems(Update, apply_interaction_palette);
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        StateScoped(GameState::StartMenu),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            get_state_transition_button("New Game", GameState::GameSelection),
            get_state_transition_button("Load Game", GameState::LoadGame),
            get_state_transition_button("Map Editor", GameState::MapEditor),
            get_state_transition_button("Hero Editor", GameState::HeroEditor),
            get_state_transition_button("Options", GameState::Options),
            get_state_transition_button("Credits", GameState::Credits),
            get_state_transition_button("Exit", GameState::ExitingGame),
        ],
    ));
}

fn get_state_transition_button(text: impl Into<String>, next_state: GameState) -> impl Bundle {
    let text = text.into();

    (
        Name::new(format!("{}Button", text)),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            bottom: Val::Px(5.),
                            ..default()
                        },
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    InteractionPalette {
                        none: NORMAL_BUTTON,
                        hovered: HOVERED_BUTTON,
                        pressed: PRESSED_BUTTON,
                    },
                    children![
                        Text::new(text),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9))
                    ],
                ))
                .observe(
                    move |_: Trigger<Pointer<Click>>,
                          mut next_state_res: ResMut<NextState<GameState>>| {
                        next_state_res.set(next_state);
                    },
                );
        })),
    )
}
