use bevy::{
    app::{App, Startup, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Commands, Query, Res},
        world::Mut,
    },
    math::{Vec2, Vec3},
    sprite::Sprite,
    transform::components::Transform,
};

use crate::{game::grid::Core, shared::components::Health};

#[derive(Component)]
pub struct Crier;

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
        Crier,
        Health(100),
        Sprite::from_image(asset_server.load("enemies/crier.png")),
        Transform::from_xyz(pos.x, pos.y, 5.),
    ));
}

/// Enemy moves straight to core, it can go past towers
/// Use without to make sure transforms don't overlap
pub fn move_enemy(
    mut core_transform: Query<&Transform, (Without<Crier>, With<Core>)>,
    enemy_query: Query<&mut Transform, With<Crier>>,
) {
    let core_pos = core_transform.single_mut().unwrap().translation;
    for transform in enemy_query {
        move_to_nearest_defence(transform.translation, core_pos, transform)
    }
}

fn move_to_nearest_defence(enemy_pos: Vec3, core_pos: Vec3, mut transform: Mut<'_, Transform>) {
    let hypotenuse_vec = Vec2 {
        x: enemy_pos.x - core_pos.x,
        y: enemy_pos.y - core_pos.y,
    };

    let normalized_vec = hypotenuse_vec.normalize();
    transform.translation.x += -normalized_vec.x / 2.;
    transform.translation.y += -normalized_vec.y / 2.;
}
