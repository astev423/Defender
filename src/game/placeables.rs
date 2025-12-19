use bevy::{
    app::{App, Update},
    asset::AssetServer,
    ecs::system::{Commands, Res},
    math::Vec2,
    sprite::Sprite,
    transform::components::Transform,
};

pub fn placeables_plugin(app: &mut App) {
    app.add_systems(Update, search_for_enemies);
}

pub fn place_tower(mut commands: Commands, pos: Vec2, asset_server: Res<AssetServer>) {
    let tower = asset_server.load("defences/tower1.png");
    commands.spawn((
        Sprite::from_image(tower),
        Transform::from_xyz(pos.x - 620., -pos.y + 360., 1.),
    ));
}

pub fn search_for_enemies(query) {
    // For tower in towers placed
        // Get range of current tower
        // For enemy in enemy
            // See if enemy position is in range, if it is then attack enemy with tower's damage
}

fn attack_enemy(_damage: i32) {}
