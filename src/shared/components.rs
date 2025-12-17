use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Damage(pub i32);
