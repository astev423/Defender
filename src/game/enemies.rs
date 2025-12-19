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
    time::Time,
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

impl Enemy {
    pub fn get_damage(&self) -> f32 {
        match self.kind {
            EnemyType::Crier => return 30.,
            EnemyType::Gazer => return 40.,
        }
    }
    pub fn get_speed(&self) -> f32 {
        match self.kind {
            EnemyType::Crier => return 40.,
            EnemyType::Gazer => return 50.,
        }
    }
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
    time: Res<Time>,
    mut core_transform: Query<&Transform, (Without<Enemy>, With<Core>)>,
    enemies: Query<(&mut Transform, &Enemy)>,
) {
    let delta = time.delta_secs();
    let core_pos = core_transform.single_mut().unwrap().translation;
    for (mut transform, enemy) in enemies {
        move_to_nearest_defence(delta, core_pos, &mut transform.translation, enemy)
    }
}

fn move_to_nearest_defence(delta: f32, core_pos: Vec3, enemy_pos: &mut Vec3, enemy: &Enemy) {
    let hypotenuse_vec = Vec2 {
        x: enemy_pos.x - core_pos.x,
        y: enemy_pos.y - core_pos.y,
    };

    let velocity = hypotenuse_vec.normalize() * delta * enemy.get_speed();
    enemy_pos.x += -velocity.x / 2.;
    enemy_pos.y += -velocity.y / 2.;
}
