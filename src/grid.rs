use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct Tile {
    spawn_pos_xy: (f32, f32),
}

pub fn grid_plugin(app: &mut App) {
    app.add_systems(Startup, make_grid)
        .add_systems(Update, modify_clicked_tile);
}

pub fn make_grid(mut commands: Commands) {
    for x in (41..1230).step_by(41).map(|x| x as f32) {
        for y in (41..655).step_by(41).map(|x| x as f32) {
            // Spawn takes in a bundle of components which are data, entity itself has no data
            commands.spawn((
                Tile {
                    spawn_pos_xy: (x, y),
                },
                Sprite::from_color(Color::linear_rgb(0., 255., 0.), Vec2 { x: 40., y: 40. }),
                Transform::from_xyz(x - 620., -y + 340., 0.),
            ));
        }
    }
}

fn match_click_to_tile(windows: Query<&Window, With<PrimaryWindow>>) -> (f32, f32) {
    let mouse_pos = windows.single().unwrap().cursor_position().unwrap();
    let x_pos_remainder = mouse_pos.x % 41.;
    let x_pos = mouse_pos.x - x_pos_remainder;
    let y_pos_remainder = mouse_pos.y % 41.;
    let y_pos = mouse_pos.y - y_pos_remainder;
    (x_pos, y_pos)
    //println!("matching x pos for tile starting at: {:?}", x_pos);
    //println!("matching y pos for tile starting at: {:?}", y_pos);
}

pub fn modify_clicked_tile(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    // This matches the tile to its sprite
    mut query: Query<(&mut Tile, &mut Sprite)>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let spawn_pos_of_clicked_tile = match_click_to_tile(windows);
    for (tile, mut sprite) in query.iter_mut() {
        if tile.spawn_pos_xy == spawn_pos_of_clicked_tile {
            sprite.color = Color::srgb(255., 0., 0.);
            return;
        }
    }
}
