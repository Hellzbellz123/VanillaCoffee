use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_ecs_ldtk::{GridCoords, IntGridCell, LayerMetadata};
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use heron::CollisionEvent;
use std::time::Duration;

use crate::{
    action_manager::actions::PlayerBindables,
    actors::{
        animation::{AnimState, AnimationSheet, FacingDirection},
        ActorState,
    },
    game::TimeInfo,
    // game_world::world_components::Collides,
    AppSettings,
};

mod debug_dirs;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        debug_dirs::debugdir();
        let _registry = app
            .world
            .get_resource_or_insert_with(bevy_inspector_egui::InspectableRegistry::default);

        app.add_plugin(InspectorPlugin::<AppSettings>::new())
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin {
                wait_duration: Duration::from_secs(20),
                ..Default::default()
            })
            //rapier inspectables in this plugin
            .add_plugin(InspectableRapierPlugin)
            //custom inspectables not from plugins
            .register_inspectable::<ActorState>()
            .register_type::<TimeInfo>()
            .register_type::<AnimState>()
            .register_inspectable::<AnimationSheet>()
            .register_inspectable::<FacingDirection>() // tells bevy-inspector-egui how to display the struct in the world inspector
            // .register_inspectable::<Collides>()
            .register_type::<PlayerBindables>()
            // LDTK debug data
            .register_type::<LayerMetadata>()
            .register_type::<IntGridCell>()
            .register_type::<GridCoords>();
        // .add_system(debug_collision_events);
    }
}

// fn log_collisions(mut events: EventReader<CollisionEvent>) {
//     for event in events.iter() {
//         if event.is_started() {
//             info!("{:?}", event);
//         }
//     }
// }

// fn debug_collision_events(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
//     events
//         .iter()
//         // We care about when the entities "start" to collide
//         .filter(|e| e.is_started())
//         .filter_map(|event| {
//             let (entity_1, entity_2) = event.rigid_body_entities();
//             let (layers_1, layers_2) = event.collision_layers();
//             if is_player(layers_1) && is_enemy(layers_2) | is_player(layers_2) && is_enemy(layers_1)
//             {
//                 info!("player and enemy collided");
//                 Some(entity_1)
//             } else if is_player(layers_2) && is_sensor(layers_1)
//                 || is_player(layers_1) && is_sensor(layers_2)
//             {
//                 info!("player and sensor collided");
//                 layers_1.groups_bits();
//                 Some(entity_1)
//             } else {
//                 info!("not player or enemy or sensor, we can ignore");
//                 // This event is not the collision between an enemy and the player. We can ignore it.
//                 None
//             }
//         })
//         .for_each(|entity| {
//             // let player = entity.id();
//             info!("{}", entity.id());
//         })
// }
