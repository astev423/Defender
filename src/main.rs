// first make a grid, then let users place things on that grid

use bevy::{prelude::*, window::WindowResolution};
use defender::{components::Health, enemies::enemy_plugin, grid::grid_plugin, ui::ui_plugin};

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
        .add_systems(Startup, spawn_cam_and_core) // run once at the beginning
        .run();
}

// For queries to work we need to make this a component
#[derive(Component)]
struct Core;

fn spawn_cam_and_core(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut transform = Transform::from_xyz(0., 0., 1.);
    transform.scale = Vec3 {
        x: 2.,
        y: 2.,
        z: 1.,
    };
    commands.spawn(Camera2d::default());
    commands.spawn((
        Core,
        Health(10000),
        Sprite::from_image(asset_server.load("core.png")),
        transform,
    ));
}
