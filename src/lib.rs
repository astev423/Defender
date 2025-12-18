pub mod game;
pub mod shared;
pub mod ui;

use crate::{
    game::{enemies::enemy_plugin, grid::grid_plugin},
    ui::money::ui_plugin,
};
use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ui_plugin, grid_plugin, enemy_plugin))
            .add_systems(Startup, |mut commands: Commands| {
                commands.spawn(Camera2d::default());
            });
    }
}
