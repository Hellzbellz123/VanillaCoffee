use bevy::prelude::*;

pub mod homeworld;
pub mod world_components;

pub struct MapSystem;

impl Plugin for MapSystem {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(homeworld::HomeWorldPlugin);
    }
}
