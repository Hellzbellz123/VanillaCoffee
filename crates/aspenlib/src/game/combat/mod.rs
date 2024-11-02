use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier2d::{
    geometry::SolverFlags,
    pipeline::{BevyPhysicsHooks, PairFilterContextView},
};

use crate::{
    game::{
        attributes_stats::{CharacterStats, DamageQueue},
        characters::player::PlayerSelectedHero,
        combat::unarmed::EventAttackUnarmed,
        game_world::{
            components::{ActorTeleportEvent, TpTriggerEffect},
            dungeonator_v2::GeneratorState,
            RegenReason, RegenerateDungeonEvent,
        },
        items::weapons::{
            components::{WeaponDescriptor, WeaponHolder},
            EventAttackWeapon,
        },
        progress::CurrentRunInformation,
    },
    utilities::EntityCreator,
    AppStage,
};

/// handles attacks from characters without weapons
pub mod unarmed;

/// game combat functionality
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(unarmed::UnArmedPlugin);

        app.add_event::<EventRequestAttack>();

        app.add_systems(
            PreUpdate,
            apply_damage_system.run_if(in_state(AppStage::Running)),
        );

        app.add_systems(
            Update,
            (
                delegate_attack_events.run_if(on_event::<EventRequestAttack>()),
                handle_death_system,
            )
                .run_if(in_state(AppStage::Running)),
        );
    }
}

// TODO: have damaged characters use particle effect or red tint when damaged
/// applys
#[allow(clippy::type_complexity)]
fn apply_damage_system(
    mut game_info: ResMut<CurrentRunInformation>,
    mut damaged_characters: Query<
        (&mut CharacterStats, Entity, &mut DamageQueue),
        Changed<DamageQueue>,
    >,
    player_controlled: Query<&PlayerSelectedHero>,
) {
    for (mut character_stats, character, mut damage_queue) in &mut damaged_characters {
        for damage in damage_queue.iter_queue() {
            if character_stats.get_current_health() <= 0.0 {
                return;
            }
            if player_controlled.get(character).is_ok() {
                game_info.player_physical_damage_taken += damage.physical.0;
            } else {
                game_info.enemy_physical_damage_taken += damage.physical.0;
            }
            character_stats.apply_damage(*damage);
        }
        damage_queue.empty_queue();
    }
}

/// gathers entitys that have damage and despawns them if have no remaining health
#[allow(clippy::type_complexity)]
fn handle_death_system(
    mut game_info: ResMut<CurrentRunInformation>,
    mut cmds: Commands,
    mut damaged_query: Query<
        (
            Entity,
            &mut CharacterStats,
            &mut Transform,
            Option<&PlayerSelectedHero>,
        ),
        Changed<CharacterStats>,
    >,
    dungeon_state: Res<State<GeneratorState>>,
    mut regen_event: EventWriter<RegenerateDungeonEvent>,
    mut tp_event: EventWriter<ActorTeleportEvent>,
) {
    for (ent, mut stats, _transform, player_control) in &mut damaged_query {
        if stats.get_current_health() <= 0.0 {
            // should probably despawn player and rebuild.
            // or auto use postion and if dead restart
            if player_control.is_some() {
                info!("player died, resetting player");
                stats.set_health(150.0);
                game_info.player_deaths += 1;

                if *dungeon_state.get() == GeneratorState::FinishedDungeonGen {
                    regen_event.send(RegenerateDungeonEvent {
                        reason: RegenReason::PlayerDeath,
                    });
                } else {
                    tp_event.send(ActorTeleportEvent {
                        tp_type: TpTriggerEffect::Event("TeleportStartLocation".to_string()),
                        target: Some(ent),
                        sender: Some(ent),
                    });
                }
                continue;
            }

            // entity that died is not player
            error!("despawning entity");
            game_info.enemies_deaths += 1;
            cmds.entity(ent).despawn_recursive();
        }
    }
}

/// triggers weapon attacks if weapon weapon exists
fn delegate_attack_events(
    mut attack_events: EventReader<EventRequestAttack>,
    mut weapon_attack_events: EventWriter<EventAttackWeapon>,
    mut unarmed_attack_events: EventWriter<EventAttackUnarmed>,
    weapon_query: Query<(&WeaponDescriptor, &WeaponHolder), With<Parent>>,
) {
    for attack_request in attack_events.read() {
        match attack_request.direction {
            AttackDirection::FromWeapon(weapon_id) => {
                let Ok((_weapon_info, _weapon_holder)) = weapon_query.get(weapon_id) else {
                    warn!("attack event received but weapon is missing important components");
                    continue;
                };

                weapon_attack_events.send(EventAttackWeapon {
                    requester: attack_request.requester,
                    weapon: weapon_id,
                });
            }
            AttackDirection::FromVector(attack_direction) => {
                unarmed_attack_events.send(EventAttackUnarmed {
                    requester: attack_request.requester,
                    direction: attack_direction,
                });
            }
        };
    }
}

/// character wanted attack
#[derive(Debug, Event)]
pub struct EventRequestAttack {
    /// who is using the weapon
    pub requester: Entity,
    /// what weapon is this attacker using
    pub direction: AttackDirection,
}

/// how too get direction this attack request is towards
#[derive(Debug)]
pub enum AttackDirection {
    /// weapon attack direction is collected from weapons rotation
    FromWeapon(Entity),
    /// weapon attack direction is calculated from a target position
    FromVector(Vec2),
}

/// A custom filter that ignores contacts if both contact entities share the same '`EntityCreator`'
#[derive(SystemParam)]
pub struct SameUserDataFilter<'w, 's> {
    /// tags for filtering
    tags: Query<'w, 's, &'static EntityCreator>,
}

impl BevyPhysicsHooks for SameUserDataFilter<'_, '_> {
    fn filter_contact_pair(&self, context: PairFilterContextView) -> Option<SolverFlags> {
        if let Some(a_filter) = self.tags.get(context.collider1()).ok()
            && let Some(b_filter) = self.tags.get(context.collider2()).ok()
            && a_filter.0 == b_filter.0
        {
            // this bullet was requested by opposite entitity
            // dont 'hit' it.
            return None;
        }

        Some(SolverFlags::COMPUTE_IMPULSES)
    }
}
