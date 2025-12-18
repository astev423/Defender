// first make a grid, then let users place things on that grid

use bevy::{prelude::*, window::WindowResolution};
use defender::AppPlugin;

/// Default plugins sets up bevy logic and fixes window size, AppPlugin is my game logic
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Defender".to_string(),
                    resolution: WindowResolution::default(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            AppPlugin,
        ))
        .run();
}
