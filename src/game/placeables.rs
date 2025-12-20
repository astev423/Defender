use bevy::{
    app::{App, Update},
    asset::AssetServer,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    gizmos::gizmos::Gizmos,
    math::{Vec2, Vec3},
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
};

use crate::{game::enemies::Enemy, shared::components::Health};

pub enum TowerName {
    Shockah,
    Burnah,
}

#[derive(Component)]
pub struct Tower {
    pub name: TowerName,
    pub range: f32,
    pub damage: f32,
    pub max_targets: i32,
}

impl Tower {
    pub fn new(name: &TowerName) -> Tower {
        match name {
            TowerName::Shockah => {
                return Tower {
                    name: TowerName::Shockah,
                    range: 300.,
                    damage: 1.5,
                    max_targets: 1,
                };
            }
            TowerName::Burnah => {
                return Tower {
                    name: TowerName::Burnah,
                    range: 50.,
                    damage: 5.,
                    max_targets: 5,
                };
            }
        }
    }
}

pub fn placeables_plugin(app: &mut App) {
    app.add_systems(Update, search_for_enemies);
}

pub fn place_tower(
    chosen_tower: &TowerName,
    mut commands: Commands,
    pos: Vec2,
    asset_server: Res<AssetServer>,
) {
    let chosen_tower_str = match chosen_tower {
        TowerName::Shockah => "shockah",
        TowerName::Burnah => "burnah",
    };

    let tower = asset_server.load(format!("defences/{chosen_tower_str}.png"));
    commands.spawn((
        Sprite::from_image(tower),
        Transform::from_xyz(pos.x - 620., -pos.y + 360., 1.),
        Tower::new(chosen_tower),
    ));
}

/// Loop through all towers, each time checking if enemy in range and attacking if they are
pub fn search_for_enemies(
    time: Res<Time>,
    towers: Query<(&mut Transform, &mut Tower), Without<Enemy>>,
    mut enemies: Query<(&mut Transform, Entity, &mut Health), With<Enemy>>,
    mut commands: Commands,
    mut gizmos: Gizmos,
) {
    for (tower_transform, tower) in towers.iter() {
        let tower_pos = tower_transform.translation;
        let mut enemies_attacked = 0;
        for (enemy_transform, enemy_entity, mut health) in enemies.iter_mut() {
            let enemy_pos = enemy_transform.translation;
            if enemies_attacked >= tower.max_targets {
                break;
            }
            if is_enemy_in_range(tower, &tower_pos, &enemy_pos) {
                reduce_enemy_health(tower, &time, enemy_entity, &mut commands, &mut health.0);
                do_attack_animation(tower, &mut gizmos, &tower_pos, &enemy_pos);
                enemies_attacked += 1;
            }
        }
    }
}

fn is_enemy_in_range(tower: &Tower, tower_pos: &Vec3, enemy_pos: &Vec3) -> bool {
    let range = tower.range;
    let dist_between_entities = Vec2 {
        x: (tower_pos.x - enemy_pos.x).abs(),
        y: (tower_pos.y - enemy_pos.y).abs(),
    }
    .length();

    if dist_between_entities <= range {
        return true;
    }

    false
}

fn reduce_enemy_health(
    tower: &Tower,
    time: &Res<'_, Time>,
    enemy_entity: Entity,
    commands: &mut Commands,
    health: &mut f32,
) {
    let delta = time.delta_secs();
    let damage = tower.damage * delta;
    *health -= damage;

    // Despawn if health low enough
    if *health <= 0. {
        commands.entity(enemy_entity).try_despawn();
    }
}

fn do_attack_animation(tower: &Tower, gizmos: &mut Gizmos, tower_pos: &Vec3, enemy_pos: &Vec3) {
    match tower.name {
        TowerName::Shockah => zap_enemy_animation(gizmos, &tower_pos, &enemy_pos),
        TowerName::Burnah => burn_enemy_animation(gizmos, &tower_pos, &enemy_pos),
    }
}

/// This draws a yellow line between two points
fn zap_enemy_animation(gizmos: &mut Gizmos, tower_pos: &Vec3, enemy_pos: &Vec3) {
    let tower_point = Vec2 {
        x: tower_pos.x,
        y: tower_pos.y,
    };
    let enemy_point = Vec2 {
        x: enemy_pos.x,
        y: enemy_pos.y,
    };

    gizmos.line_2d(tower_point, enemy_point, Color::hsl(62., 1., 0.5));
}

fn burn_enemy_animation(gizmos: &mut Gizmos, tower_pos: &Vec3, enemy_pos: &Vec3) {}
