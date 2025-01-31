use avian2d::prelude::{CollisionLayers, LayerMask};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    bundles::{Aspen2dPhysicsBundle, AspenColliderBundle, NeedsCollider},
    consts::{AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX},
    game::components::ActorColliderType,
    loading::{
        custom_assets::actor_definitions::ItemDefinition,
        registry::{ActorRegistry, RegistryIdentifier},
    },
};

/// spawns weapon item
pub fn spawn_weapon(
    registry: &Res<ActorRegistry>,
    item_assets: &Res<Assets<ItemDefinition>>,
    spawn_data: &(RegistryIdentifier, i32),
    spawn_position: Vec2,
    commands: &mut Commands,
) {
    let (_, _item_def) = item_assets
        .iter()
        .find(|(_, asset)| asset.actor.identifier == spawn_data.0)
        .expect("Spawned characters asset definition did not exist");

    let Some(weapon_bundle) = registry.items.weapons.get(&spawn_data.0) else {
        error!(
            "could not get WeaponBundle from registry: {:?}",
            &spawn_data.0
        );
        return;
    };

    let mut rng = thread_rng();
    for _ in 0..spawn_data.1 {
        let position = Vec2 {
            x: spawn_position.x + rng.gen_range(-100.0..=100.0),
            y: spawn_position.y + rng.gen_range(-100.0..=100.0),
        };

        info!("spawning weapon");
        commands
            .spawn((
                Transform::from_translation(position.extend(ACTOR_Z_INDEX)),
                weapon_bundle.clone(),
                Aspen2dPhysicsBundle::default_item(),
            ))
            .with_children(|child| {
                let collider_name = format!("{}Collider", weapon_bundle.name.as_str());
                // TODO: change so all collider bundles are constified
                child.spawn(AspenColliderBundle {
                    tag: ActorColliderType::Item,
                    name: Name::new(collider_name),
                    collider: NeedsCollider::Aabb,
                    collision_groups: CollisionLayers::new(
                        AspenCollisionLayer::DynamicActor,
                        LayerMask::ALL,
                    ),
                    transform: Transform {
                        translation: Vec3 {
                            x: -2.25,
                            y: -2.525,
                            z: ACTOR_PHYSICS_Z_INDEX,
                        },
                        rotation: Quat::IDENTITY,
                        scale: Vec3::ONE,
                    },
                });
            });
    }
}
