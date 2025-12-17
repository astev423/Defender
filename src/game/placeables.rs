use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    math::Vec2,
    sprite::Sprite,
    transform::components::Transform,
};

pub fn place_tower(mut commands: Commands, pos: Vec2, asset_server: Res<AssetServer>) {
    let tower = asset_server.load("defences/tower1.png");
    commands.spawn((
        Sprite::from_image(tower),
        Transform::from_xyz(pos.x - 620., -pos.y + 360., 1.),
    ));
}

pub fn search_for_enemies(_range: i32) {}

pub fn attack_enemy() {}
