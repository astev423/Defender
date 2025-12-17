use bevy::{ecs::system::SystemParam, prelude::*, window::PrimaryWindow};

use crate::{
    components::{Money, Tile},
    placeables::place_tower,
    ui::update_money,
};

pub fn grid_plugin(app: &mut App) {
    app.add_systems(Startup, make_grid)
        .add_systems(Update, modify_clicked_tile);
}

// This sets up the 28 long 14 high grid, each tile is an entity
pub fn make_grid(mut commands: Commands) {
    for x in (41..1230).step_by(41) {
        for y in (41..655).step_by(41) {
            let xf32 = x as f32;
            let yf32 = y as f32;
            commands.spawn((
                Tile {
                    spawn_pos: Vec2 { x: xf32, y: yf32 },
                    occupied: false,
                },
                Sprite::from_color(Color::linear_rgb(0., 255., 0.), Vec2 { x: 40., y: 40. }),
                Transform::from_xyz(xf32 - 620., -yf32 + 340., 0.),
            ));
        }
    }
}

fn match_click_to_tile(windows: Query<&Window, With<PrimaryWindow>>) -> Vec2 {
    let mouse_pos = windows.single().unwrap().cursor_position().unwrap();
    let x_pos_remainder = mouse_pos.x % 41.;
    let x_pos = mouse_pos.x - x_pos_remainder;
    let y_pos_remainder = mouse_pos.y % 41.;
    let y_pos = mouse_pos.y - y_pos_remainder;
    Vec2 { x: x_pos, y: y_pos }
    //println!("matching x pos for tile starting at: {:?}", x_pos);
    //println!("matching y pos for tile starting at: {:?}", y_pos);
}

#[derive(SystemParam)]
pub struct GameParams<'w, 's> {
    pub tiles: Query<'w, 's, (&'static mut Tile, &'static mut Sprite)>,
    pub money: Query<'w, 's, (&'static mut Money, &'static mut Text2d)>,
}

#[derive(SystemParam)]
pub struct InputParams<'w, 's> {
    pub mouse: Res<'w, ButtonInput<MouseButton>>,
    pub windows: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
}

/// Place things on tile clicked if user has enough money
pub fn modify_clicked_tile(
    commands: Commands,
    asset_server: Res<AssetServer>,
    input: InputParams,
    game_query: GameParams,
) {
    if !input.mouse.just_pressed(MouseButton::Left) {
        return;
    }

    // Find tile we clicked on and update it if possible
    let spawn_pos_of_clicked_tile = match_click_to_tile(input.windows);
    for (mut tile, mut sprite) in game_query.tiles {
        if tile.spawn_pos == spawn_pos_of_clicked_tile && !tile.occupied {
            if update_money(-100, game_query.money).is_err() {
                return;
            }

            sprite.color = Color::hsl(0., 1., 0.9);
            place_tower(commands, spawn_pos_of_clicked_tile, asset_server);
            tile.occupied = true;
            return;
        }
    }
}
