use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

use crate::game::actors::{
    attributes_stats::{DamageQueue, ProjectileStats},
    components::ActorColliderType,
};

/// detects projectile hits on player, adds hits too Player
pub fn projectile_hits(
    // mut game_info: ResMut<CurrentRunInformation>,
    mut cmds: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut damage_queue_query: Query<&mut DamageQueue>,
    parented_collider_query: Query<(Entity, &Parent), (With<Collider>, With<ActorColliderType>)>,
    projectile_info: Query<&ProjectileStats>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(a, b, flags) = event {
            if flags.contains(CollisionEventFlags::SENSOR) {
                return;
            }
            let hit_actor = parented_collider_query
                .get(*b)
                .or_else(|_| parented_collider_query.get(*a))
                .map(|(_collider, parent)| parent.get())
                .ok();

            let hitting_projectile = parented_collider_query
                .get(*a)
                .or_else(|_| parented_collider_query.get(*b))
                .map(|(_a, parent)| parent.get())
                .ok();

            if let Some(projectile) = hitting_projectile {
                info!("projectile hit detected");
                if let Some(actor) = hit_actor {
                    let Ok(stats) = projectile_info.get(projectile) else {
                        return;
                    };
                    let Ok(mut damage_queue) = damage_queue_query.get_mut(actor) else {
                        return;
                    };

                    damage_queue.push_damage(stats.damage);
                }
                // projectile hit something other than player
                cmds.entity(projectile).despawn_recursive();
            }
        }
    }
}
