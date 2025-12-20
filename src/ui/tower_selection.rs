use bevy::ecs::resource::Resource;

use crate::game::placeables::TowerName;

#[derive(Resource)]
pub struct ChosenTower(pub TowerName);
