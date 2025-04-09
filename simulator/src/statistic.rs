use bevy::prelude::*;

pub struct StatisticPlugin;

impl Plugin for StatisticPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StatisticStore>();
    }
}

#[derive(Resource, Default)]
struct StatisticStore;
