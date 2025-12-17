use bevy::{prelude::*, window::PrimaryWindow};

use crate::ui::{Money, place_item, update_money};

#[derive(Component)]
pub struct Tile {
    spawn_pos_xy: (f32, f32),
    occupied: bool,
}

pub fn grid_plugin(app: &mut App) {
    app.add_systems(Startup, make_grid)
        .add_systems(Update, modify_clicked_tile);
}

// This sets up the 28 long 14 high grid, each tile is an entity
pub fn make_grid(mut commands: Commands) {
    for x in (41..1230).step_by(41).map(|x| x as f32) {
        for y in (41..655).step_by(41).map(|x| x as f32) {
            commands.spawn((
                Tile {
                    spawn_pos_xy: (x, y),
                    occupied: false,
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

/// Place things on tile clicked if user has enough money
pub fn modify_clicked_tile(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    // Query all entities that have both a tile and sprite component
    mut tile_query: Query<(&mut Tile, &mut Sprite)>,
    money_query: Query<(&mut Money, &mut Text2d)>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let spawn_pos_of_clicked_tile = match_click_to_tile(windows);
    for (mut tile, mut sprite) in tile_query.iter_mut() {
        if tile.spawn_pos_xy == spawn_pos_of_clicked_tile && !tile.occupied {
            let result = update_money(-100, money_query);
            if result.is_err() {
                return;
            }

            sprite.color = Color::srgb(255., 0., 0.);
            place_item();
            tile.occupied = true;
            return;
        }
    }
}
