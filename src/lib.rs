pub mod game;
pub mod shared;
pub mod ui;

use crate::{
    game::{enemies::enemy_plugin, grid::grid_plugin, placeables::placeables_plugin},
    shared::systems::debug_plugin,
    ui::ui_plugin,
};
use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ui_plugin, grid_plugin, enemy_plugin, placeables_plugin))
            // only for debug info
            .add_plugins(debug_plugin)
            .add_systems(Startup, |mut commands: Commands| {
                commands.spawn(Camera2d::default());
            });
    }
}
