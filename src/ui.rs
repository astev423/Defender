use bevy::{
    ecs::{component::Component, system::Query},
    sprite::Text2d,
};

#[derive(Component)]
pub struct Money {
    pub amount: i32,
}

pub fn decrease_money(amount: i32, mut query: Query<(&mut Money, &mut Text2d)>) {
    for (mut money, mut text) in query.iter_mut() {
        money.amount -= amount;
        text.0 = money.amount.to_string();
    }
}
