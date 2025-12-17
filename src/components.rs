use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct Tile {
    pub spawn_pos: Vec2,
    pub occupied: bool,
}

#[derive(Component)]
pub struct Money {
    pub amount: i32,
}

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Damage(pub i32);

pub mod enemies {
    use bevy::ecs::component::Component;

    #[derive(Component)]
    pub struct Crier;
}
