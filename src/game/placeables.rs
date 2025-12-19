use bevy::{
    app::{App, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        query::Without,
        system::{Commands, Query, Res},
        world::Mut,
    },
    math::{Vec2, Vec3},
    sprite::Sprite,
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
}

impl Tower {
    pub fn get_range(&self) -> f32 {
        match self.name {
            TowerName::Shockah => return 140.,
            TowerName::Burnah => return 70.,
        }
    }

    pub fn get_damage(&self) -> f32 {
        match self.name {
            TowerName::Shockah => return 70.,
            TowerName::Burnah => return 140.,
        }
    }
}

pub fn placeables_plugin(app: &mut App) {
    app.add_systems(Update, search_for_enemies);
}

pub fn place_tower(mut commands: Commands, pos: Vec2, asset_server: Res<AssetServer>) {
    let tower = asset_server.load("defences/tower1.png");
    commands.spawn((
        Sprite::from_image(tower),
        Transform::from_xyz(pos.x - 620., -pos.y + 360., 1.),
        Tower {
            name: TowerName::Shockah,
        },
    ));
}

pub fn search_for_enemies(
    towers: Query<(&mut Transform, &mut Tower), Without<Enemy>>,
    mut enemies: Query<(&mut Transform, &mut Enemy, &mut Health)>,
) {
    for (tower_pos, tower) in towers.iter() {
        for (enemy_pos, enemy, health) in enemies.iter_mut() {
            if is_enemy_in_range(tower, &tower_pos.translation, &enemy_pos.translation) {
                attack_enemy(tower, enemy, health);
            }
        }
    }
}

fn is_enemy_in_range(tower: &Tower, tower_pos: &Vec3, enemy_pos: &Vec3) -> bool {
    let range = tower.get_range();
    let dist_between_entities = Vec2 {
        x: (tower_pos.x - enemy_pos.x).abs(),
        y: (tower_pos.y - enemy_pos.y).abs(),
    }
    .length();

    if dist_between_entities <= range {
        println!("Enemy in range!!!");
        return true;
    }

    false
}

fn attack_enemy(tower: &Tower, enemy: Mut<'_, Enemy>, mut health: Mut<'_, Health>) {
    // need delta
    let damage = tower.get_damage();
    health.0 -= damage;
    println!("{:?}", health.0);
}
