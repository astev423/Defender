use bevy::{
    app::{App, Startup, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    math::{Vec2, Vec3},
    sprite::Sprite,
    transform::components::Transform,
};

use crate::{game::grid::Core, shared::components::Health};

pub enum EnemyType {
    Crier,
    Gazer,
}

#[derive(Component)]
pub struct Enemy {
    pub kind: EnemyType,
}

pub fn enemy_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_enemies)
        .add_systems(Update, move_enemy);
}

pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_enemy(&mut commands, Vec2 { x: -400., y: 200. }, &asset_server);
    spawn_enemy(&mut commands, Vec2 { x: 400., y: -200. }, &asset_server);
    spawn_enemy(&mut commands, Vec2 { x: 500., y: 0. }, &asset_server);
    spawn_enemy(&mut commands, Vec2 { x: 500., y: 100. }, &asset_server);
}

pub fn spawn_enemy(commands: &mut Commands, pos: Vec2, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Enemy {
            kind: EnemyType::Crier,
        },
        Health(100.),
        Sprite::from_image(asset_server.load("enemies/crier.png")),
        Transform::from_xyz(pos.x, pos.y, 5.),
    ));
}

/// Enemy moves straight to core, it can go past towers
/// Use without to make sure transforms don't overlap
pub fn move_enemy(
    mut core_transform: Query<&Transform, (Without<Enemy>, With<Core>)>,
    enemy_locations: Query<&mut Transform, With<Enemy>>,
) {
    let core_pos = core_transform.single_mut().unwrap().translation;
    for mut location in enemy_locations {
        move_to_nearest_defence(location.translation, core_pos, &mut location.translation)
    }
}

fn move_to_nearest_defence(enemy_pos: Vec3, core_pos: Vec3, transform: &mut Vec3) {
    let hypotenuse_vec = Vec2 {
        x: enemy_pos.x - core_pos.x,
        y: enemy_pos.y - core_pos.y,
    };

    let normalized_vec = hypotenuse_vec.normalize();
    transform.x += -normalized_vec.x / 2.;
    transform.y += -normalized_vec.y / 2.;
}
