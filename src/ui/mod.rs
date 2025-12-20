use bevy::app::{App, Startup};

use crate::{
    game::placeables::TowerName,
    ui::{money::spawn_money, tower_selection::ChosenTower},
};

pub mod money;
pub mod tower_selection;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_money)
        .insert_resource(ChosenTower(TowerName::Burnah));
}
