use bevy::{
    app::{App, Startup, Update},
    asset::AssetServer,
    color::Color,
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    gizmos::gizmos::Gizmos,
    math::{Vec2, Vec3},
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
};
use rand::Rng;

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
            EnemyType::Crier => return 30.,
            EnemyType::Gazer => return 40.,
        }
    }
}

pub fn enemy_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_enemies)
        .add_systems(Update, move_enemy);
}

// Spawn 10 enemies randomly
pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let min_pos = Vec2::new(-300., -200.);
    let max_pos = Vec2::new(802., 330.);

    let mut rng = rand::rng();
    for _ in 0..10 {
        let pos = Vec2::new(
            rng.random_range(min_pos.x..=max_pos.x),
            rng.random_range(min_pos.y..=max_pos.y),
        );
        spawn_enemy(&mut commands, pos, &asset_server);
    }
}

pub fn spawn_enemy(commands: &mut Commands, pos: Vec2, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Enemy {
            kind: EnemyType::Crier,
        },
        Health(50.),
        Sprite::from_image(asset_server.load("enemies/crier.png")),
        Transform::from_xyz(pos.x, pos.y, 5.),
    ));
}

/// Enemy moves straight to core, it can go past towers
/// Use without to make sure transforms don't overlap
pub fn move_enemy(
    time: Res<Time>,
    mut core_transform: Query<&Transform, (Without<Enemy>, With<Core>)>,
    enemies: Query<(&mut Transform, &Enemy, &Health)>,
    mut gizmos: Gizmos,
) {
    let delta = time.delta_secs();
    let core_pos = core_transform.single_mut().unwrap().translation;

    for (mut transform, enemy, health) in enemies {
        let enemy_pos = &mut transform.translation;
        move_to_nearest_defence(delta, core_pos, enemy_pos, enemy);
        update_health_bar(enemy_pos, health.0, &mut gizmos);
    }
}

fn update_health_bar(enemy_pos: &mut Vec3, health: f32, gizmos: &mut Gizmos) {
    let health_bar_size = health / 2.;
    let line_start = Vec2 {
        x: enemy_pos.x - health_bar_size,
        y: enemy_pos.y + 30.,
    };
    let line_end = Vec2 {
        x: enemy_pos.x + health_bar_size,
        y: enemy_pos.y + 30.,
    };
    gizmos.line_2d(line_start, line_end, Color::srgb(1., 0., 0.));
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
