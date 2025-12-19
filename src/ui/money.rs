use bevy::{
    app::{App, Startup},
    ecs::{
        query::With,
        system::{Commands, Query, Res},
    },
    input::{ButtonInput, mouse::MouseButton},
    sprite::Text2d,
    text::{TextFont},
    transform::components::Transform,
    window::{PrimaryWindow, Window},
};

use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Money {
    pub amount: i32,
}

pub fn ui_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_money);
}

pub fn spawn_money(mut commands: Commands) {
    let money = Money { amount: 1000 };
    commands.spawn((
        Text2d::new(format!("Money: {}", money.amount)),
        TextFont {
            font_size: 25.,
            ..TextFont::default()
        },
        money,
        Transform::from_xyz(-540., 340., 0.),
    ));
}

/// Subtract building cost placed from money if user has enough money, otherwise err
pub fn update_money<'a>(
    amount: i32,
    mut money_query: Query<'_, '_, (&mut Money, &mut Text2d)>,
) -> Result<(), &'a str> {
    let money = &mut money_query
        .single_mut()
        .expect("There are multiple money structs, that shouldn't happen!");
    let money_amount = &mut money.0.amount;
    if *money_amount + amount < 0 {
        return Err("Not enough money to place item!");
    }

    *money_amount += amount;
    let money_text = &mut money.1.0;
    *money_text = format!("Money: {money_amount}");

    Ok(())
}

pub fn check_placements_or_selections(
    _mouse: Res<ButtonInput<MouseButton>>,
    _windows: Query<&Window, With<PrimaryWindow>>,
    // Query all entities that have both a tile and sprite component
) {
}
