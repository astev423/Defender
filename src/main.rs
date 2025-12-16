// first make a grid, then let users place things on that grid

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use defender::{
    grid::{grid_plugin, make_grid, modify_clicked_tile},
    ui::{Money, decrease_money},
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
        .add_systems(Startup, setup) // run once at the beginning
        .add_systems(Update, modify_clicked_tile)
        .run();
}

// For queries to work we need to make this a component
#[derive(Component)]
struct Core;

fn setup(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let money = Money { amount: 1000 };
    commands.spawn(Camera2d::default());
    commands.spawn((
        Text2d::new(format!("{}", money.amount)),
        money,
        Transform::from_xyz(-500., 200., 0.),
    ));
    commands.spawn((
        Core,
        Sprite::from_color(Color::WHITE, Vec2 { x: 40.0, y: 40.0 }),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}
