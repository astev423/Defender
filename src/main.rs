// first make a grid, then let users place things on that grid

use bevy::{prelude::*, window::WindowResolution};
use defender::{
    game::{enemies::enemy_plugin, grid::grid_plugin},
    ui::money::ui_plugin,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Defender".to_string(),
                resolution: WindowResolution::default(),
                resizable: false,
                ..default()
            }),
            ..default()
        }),))
        .add_plugins(grid_plugin)
        .add_plugins(ui_plugin)
        .add_plugins(enemy_plugin)
        .add_systems(Startup, spawn_cam) // run once at the beginning
        .run();
}

fn spawn_cam(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
