use avian2d::prelude::{Collider, CollisionStarted};
use bevy::prelude::*;

use crate::game::{
    attributes_stats::{DamageQueue, ProjectileStats},
    components::ActorColliderType,
};

/// detects projectile hits, adds damage too hit actors
pub fn projectile_hits(
    mut cmds: Commands,
    mut collision_events: EventReader<CollisionStarted>,
    mut damage_queue_query: Query<&mut DamageQueue>,
    actor_colliders: Query<(Entity, &Parent, &ActorColliderType), With<Collider>>,
    projectiles: Query<&ProjectileStats>,
) {
    for event in collision_events.read() {
        let CollisionStarted(a_id, b_id) = *event;

        let Some((Ok(stats), projectile)) = ({
            let mut projectile_colliders = actor_colliders
                .iter()
                .filter(|(_, _, at)| at == &&ActorColliderType::Projectile);
            projectile_colliders
                .find(|f| f.0 == a_id || f.0 == b_id)
                .map(|f| f.1.get())
                .map(|f| (projectiles.get(f), f))
        }) else {
            continue;
        };

        let Some(hit_actor) = ({
            let mut character_colliders = actor_colliders
                .iter()
                .filter(|(_, _, at)| at == &&ActorColliderType::Character);
            character_colliders
                .find(|f| f.0 == b_id || f.0 == a_id)
                .map(|f| f.1.get())
        }) else {
            // despawn projectile, hit something other than character
            cmds.entity(projectile).despawn_recursive();
            continue;
        };

        if stats.entity_that_shot == hit_actor {
            continue;
        }

        info!("projectile hit detected");
        // get hit actors damage queue
        let Ok(mut damage_queue) = damage_queue_query.get_mut(hit_actor) else {
            return;
        };

        // add damage too hit actors damage queueu
        damage_queue.push_damage(stats.damage);

        // despawn projectile
        cmds.entity(projectile).despawn_recursive();
    }
}
