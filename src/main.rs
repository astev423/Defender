// first make a grid, then let users place things on that grid

use bevy::{prelude::*, window::WindowResolution};
use defender::{grid::grid_plugin, ui::ui_plugin};

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
        .add_systems(Startup, spawn_cam_and_core) // run once at the beginning
        .run();
}

// For queries to work we need to make this a component
#[derive(Component)]
struct Core;

fn spawn_cam_and_core(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Core,
        Sprite::from_color(Color::WHITE, Vec2 { x: 40.0, y: 40.0 }),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}
