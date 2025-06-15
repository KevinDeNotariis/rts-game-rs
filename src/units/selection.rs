use bevy::prelude::*;

use crate::game_states::GameState;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SelectionState>()
            .add_systems(OnEnter(SelectionState::Selecting), create_selection_box)
            .add_systems(
                Update,
                (
                    mouse_click.run_if(in_state(GameState::Playing)),
                    update_selection_box.run_if(
                        in_state(GameState::Playing).and(in_state(SelectionState::Selecting)),
                    ),
                ),
            );
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct SelectionBox {
    origin: Vec2,
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum SelectionState {
    #[default]
    None,
    Selecting,
}

fn create_selection_box(mut commands: Commands, window: Single<&Window>) {
    let color = Color::srgba_u8(0, 200, 0, 50);

    let mouse_pos = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };
    info!("Spawing selection box");

    let left = Val::Px(mouse_pos.x);
    let top = Val::Px(mouse_pos.y);

    commands.spawn((
        SelectionBox {
            origin: Vec2::new(mouse_pos.x, mouse_pos.y),
        },
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(0.),
            height: Val::Px(0.),
            left: left,
            top: top,
            ..default()
        },
        BackgroundColor(color),
    ));
}

fn mouse_click(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<SelectionState>>,
    selection_box_query: Query<Entity, With<SelectionBox>>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        next_state.set(SelectionState::Selecting);
    }
    if mouse_button_input.just_released(MouseButton::Left) {
        next_state.set(SelectionState::None);
        if let Ok(selection_box) = selection_box_query.single() {
            commands.entity(selection_box).despawn();
        } else {
            return;
        }
    }
}

fn update_selection_box(
    window: Single<&Window>,
    selection_query: Single<(&mut Node, &SelectionBox)>,
) {
    let (mut node, selection_box) = selection_query.into_inner();

    let mouse_pos = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };

    let origin = selection_box.origin;

    // Calculate the rectangle bounds
    let min_x = origin.x.min(mouse_pos.x);
    let max_x = origin.x.max(mouse_pos.x);
    let min_y = origin.y.min(mouse_pos.y);
    let max_y = origin.y.max(mouse_pos.y);

    // Update the node to span from origin to mouse position
    node.left = Val::Px(min_x);
    node.top = Val::Px(min_y);
    node.width = Val::Px(max_x - min_x);
    node.height = Val::Px(max_y - min_y);
}
