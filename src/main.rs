// first make a grid, then let users place things on that grid

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use defender::grid::make_grid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Defender".to_string(),
                resolution: WindowResolution::default(),
                resizable: false,
                ..default()
            }),
            ..default()
        })) // window, renderer, core systems, etc.
        .add_systems(Startup, setup) // run once at the beginning
        .add_systems(Update, check_inputs)
        .run();
}

// For queries to work we need to make this a component
#[derive(Component)]
struct Core;

fn setup(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    commands.spawn(Camera2d::default());
    // Bundle makes an entity with characterisitcs in bundle
    commands.spawn((
        Core,
        Sprite::from_color(Color::WHITE, Vec2 { x: 40.0, y: 40.0 }),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
    make_grid(&mut commands, windows);
}

fn check_inputs(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        println!("Left mouse button clicked this frame");
        println!("{:?}", windows.single().unwrap().cursor_position().unwrap());
    }
}
