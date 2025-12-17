use bevy::{
    app::{App, Startup, Update},
    asset::AssetServer,
    ecs::system::{Commands, Res},
    math::Vec2,
    sprite::Sprite,
    transform::components::Transform,
};

use crate::components::{Health, enemies::Crier};

pub fn enemy_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_enemies)
        .add_systems(Update, move_enemy);
}

pub fn spawn_enemies(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_enemy(commands, Vec2 { x: -400., y: 200. }, asset_server);
}

pub fn spawn_enemy(mut commands: Commands, pos: Vec2, asset_server: Res<AssetServer>) {
    commands.spawn((
        Crier,
        Health(100),
        Sprite::from_image(asset_server.load("enemies/crier.png")),
        Transform::from_xyz(pos.x, pos.y, 5.),
    ));
}

/// Enemy moves straight to core, it can go past towers
pub fn move_enemy() {}
