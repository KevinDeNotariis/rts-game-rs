use bevy::prelude::*;

use crate::{
    game_states::GameState,
    menus::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
};

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartMenu), setup)
            .add_systems(Update, menu_actions.run_if(in_state(GameState::StartMenu)))
            .add_systems(OnExit(GameState::StartMenu), teardown);
    }
}

#[derive(Resource)]
struct MenuData {
    menu_entity: Entity,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct NewGame;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct LoadGame;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct MapEditor;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct HeroEditor;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Options;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Credits;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Exit;

fn setup(mut commands: Commands) {
    let menu_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
                get_button("New Game", NewGame),
                get_button("Load Game", LoadGame),
                get_button("Map Editor", MapEditor),
                get_button("Hero Editor", HeroEditor),
                get_button("Options", Options),
                get_button("Credits", Credits),
                get_button("Exit", Exit),
            ],
        ))
        .id();

    commands.insert_resource(MenuData { menu_entity });
}

fn get_button<T: Component>(text: &str, component: T) -> impl Bundle {
    (
        Button,
        component,
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
        children![
            Text::new(text),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9))
        ],
    )
}

fn menu_actions(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_event: EventWriter<AppExit>,
    new_game_query: Query<(), With<NewGame>>,
    load_game_query: Query<(), With<LoadGame>>,
    map_editor_query: Query<(), With<MapEditor>>,
    hero_editor_query: Query<(), With<HeroEditor>>,
    options_query: Query<(), With<Options>>,
    credits_query: Query<(), With<Credits>>,
    exit_query: Query<(), With<Exit>>,
) {
    for (entity, interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if new_game_query.contains(entity) {
                    *color = PRESSED_BUTTON.into();
                    next_state.set(GameState::GameSelection);
                } else if load_game_query.contains(entity) {
                    next_state.set(GameState::LoadGame);
                } else if map_editor_query.contains(entity) {
                    next_state.set(GameState::MapEditor);
                } else if hero_editor_query.contains(entity) {
                    next_state.set(GameState::HeroEditor);
                } else if options_query.contains(entity) {
                    next_state.set(GameState::Options);
                } else if credits_query.contains(entity) {
                    next_state.set(GameState::Credits);
                } else if exit_query.contains(entity) {
                    app_exit_event.write(AppExit::Success);
                }
            }
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
}

fn teardown(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.menu_entity).despawn();
}
