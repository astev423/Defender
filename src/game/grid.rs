use std::panic;

use bevy::{ecs::system::SystemParam, prelude::*, window::PrimaryWindow};

use bevy::{ecs::component::Component, math::Vec2};

use crate::game::enemies::Enemy;
use crate::game::placeables::place_tower;
use crate::shared::components::Health;
use crate::ui::money::{Money, update_money};
use crate::ui::tower_selection::ChosenTower;

#[derive(Component)]
pub struct Tile {
    pub spawn_pos: Vec2,
    pub occupied: bool,
}

#[derive(Component)]
pub struct Core;

#[derive(Component)]
pub struct CoreText;

pub fn grid_plugin(app: &mut App) {
    app.add_systems(Startup, make_grid)
        .add_systems(Update, modify_clicked_tile)
        .add_systems(Update, update_core_health);
}

pub fn spawn_core(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let mut transform = Transform::from_xyz(0., 0., 1.);
    transform.scale = Vec3 {
        x: 2.,
        y: 2.,
        z: 1.,
    };
    commands.spawn((
        Core,
        Health(3000.),
        Sprite::from_image(asset_server.load("core.png")),
        transform,
    ));

    // Display text above the core
    transform.translation.y += 70.;
    commands.spawn((
        CoreText,
        Text2d::new("Health: 10000".to_string()),
        TextColor(Color::BLACK),
        TextFont {
            font_size: 15.,
            ..TextFont::default()
        },
        transform,
    ));
}

pub fn update_core_health(
    time: Res<Time>,
    mut text: Query<&mut Text2d, With<CoreText>>,
    mut core_health: Query<&mut Health, With<Core>>,
    enemies: Query<(&Transform, &Enemy)>,
) {
    let delta = time.delta_secs();
    let mut core_health = core_health
        .single_mut()
        .expect("Why are there multiple cores with healths?");
    let text = text
        .single_mut()
        .expect("Why are there multiple cores with text?");

    display_core_health(text, &core_health);
    for (enemy_transform, enemy) in enemies {
        if is_enemy_on_core(&enemy_transform.translation) {
            decrease_core_health(&mut core_health.0, &enemy, delta);
        }
    }
}

fn is_enemy_on_core(enemy_pos: &Vec3) -> bool {
    if enemy_pos.x.abs() < 100. && enemy_pos.y.abs() < 30. {
        return true;
    }

    false
}

fn decrease_core_health(core_health: &mut f32, enemy: &Enemy, delta: f32) {
    *core_health -= enemy.get_damage() * delta;
    if *core_health <= 0. {
        panic::panic_any("YOU DIED!");
    }
}

// Text and core_health not mutable above but they are here since ownership is transfered
fn display_core_health(mut text: Mut<'_, Text2d>, core_health: &Mut<'_, Health>) {
    text.0 = format!("Health: {}", core_health.0);
}

/// This sets up the 28 long 14 high grid, each tile is an entity, core in center
pub fn make_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_core(&mut commands, asset_server);
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
    chosen_tower: Res<ChosenTower>,
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
            place_tower(
                &chosen_tower.0,
                commands,
                spawn_pos_of_clicked_tile,
                asset_server,
            );
            tile.occupied = true;
            return;
        }
    }
}
