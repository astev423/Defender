use bevy::ecs::system::Query;

use crate::shared::components::Health;

pub fn display_health_bar(query: Query<&Health>) {
    for health in query {
        println!("{:?}", health.0);
    }
}
