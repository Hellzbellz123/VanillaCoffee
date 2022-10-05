use bevy::prelude::{FromWorld, SystemLabel, Vec2, World};
use bevy_inspector_egui::Inspectable;
use heron::PhysicsLayer;

use crate::audio::SoundSettings;

pub const TILE_SIZE: Vec2 = Vec2 { x: 32.0, y: 32.0 };
pub const PLAYER_SIZE: Vec2 = Vec2::new(TILE_SIZE.x, TILE_SIZE.y * 2.0);

#[derive(PhysicsLayer)]
pub enum PhysicsLayers {
    World,
    Player,
    Enemies,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum SystemLabels {
    // spawn label for systems that query things that might not exist
    Spawn,
    InitSettings,
    UpdateSettings,
    /// everything that handles input
    Input,
    /// everything that updates player state
    Player,
    /// everything that moves things (works with transforms)
    Movement,
    /// systems that update the world map
    Map,
}

#[derive(Inspectable)]
pub struct AppSettings {
    pub sound_settings: SoundSettings,
    pub resolution: Vec2,
    pub camera_zoom: f32,
    // control_settings: PlayerInput,
}

//TODO: default app settings if its a setting it goes here, move this too settings plugin
impl FromWorld for AppSettings {
    fn from_world(_: &mut World) -> Self {
        AppSettings {
            sound_settings: SoundSettings {
                mastervolume: 0.5,
                ambiencevolume: 0.5,
                musicvolume: 0.5,
                soundvolume: 0.5,
            },
            resolution: Vec2 {
                x: 1200.0,
                y: 800.0,
            },
            camera_zoom: 1.0,
        }
    }
}
