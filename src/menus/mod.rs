use bevy::prelude::*;

use crate::menus::{game_selection::GameSelectionPlugin, start_menu::StartMenuPlugin};

pub mod game_selection;
pub mod start_menu;

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StartMenuPlugin, GameSelectionPlugin));
    }
}

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
