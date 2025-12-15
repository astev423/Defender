use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
struct Tile;

// Make grid off window size
pub fn make_grid(commands: &mut Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window_size = get_window_size(windows);
    let max_x_pos = window_size.x;
    let max_y_pos = window_size.y;
    for x in (220..max_x_pos - 180).step_by(41) {
        for y in (150..max_y_pos - 110).step_by(41) {
            // Spawn takes in a bundle of components which are data, entity itself has no data
            let x_spawn_pos = x as f32 - max_x_pos as f32 / 2.;
            let y_spawn_pos = y as f32 - max_y_pos as f32 / 2.;
            commands.spawn((
                Tile,
                Sprite::from_color(
                    Color::linear_rgb(0., 255., 0.),
                    Vec2 { x: 40., y: 40. },
                ),
                Transform::from_xyz(x_spawn_pos as f32, y_spawn_pos as f32, 0.),
            ));
        }
    }
}

fn get_window_size(windows: Query<&Window, With<PrimaryWindow>>) -> UVec2 {
    let window = windows.single().expect("More than one window, only have 1");
    println!("{:?}", window.cursor_position());
    window.physical_size()
}
