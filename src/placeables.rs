use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    math::vec3,
    sprite::Sprite,
    transform::components::Transform,
};

pub fn place_tower(mut commands: Commands, xy_pos: (f32, f32), asset_server: Res<AssetServer>) {
    let tower = asset_server.load("tower1.png");
    let mut tower_transform = Transform::from_xyz(xy_pos.0 - 620., -xy_pos.1 + 340., 1.);
    tower_transform.scale = vec3(0.08, 0.07, 1.0);
    commands.spawn((Sprite::from_image(tower), tower_transform));
}
