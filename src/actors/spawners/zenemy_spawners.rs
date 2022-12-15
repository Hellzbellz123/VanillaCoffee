use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use big_brain::thinker::Thinker;

use crate::{
    actors::enemies::skeleton::{SkeletonBundle, SlimeBundle},
    components::actors::{
        ai::{
            AIAttackTimer, AICanChase, AICanWander, AIChaseAction, AIEnemy, AIWanderAction,
            ActorType, AggroScore, TypeEnum,
        },
        animation::{AnimState, AnimationSheet, FacingDirection},
        bundles::{ActorColliderBundle, RigidBodyBundle, StupidAiBundle},
        general::MovementState,
        spawners::SpawnEnemyEvent,
    },
    loading::assets::ActorTextureHandles,
    utilities::game::{ACTOR_PHYSICS_LAYER, ACTOR_SIZE},
};

pub fn spawn_skeleton(
    enemycontainer: Entity,
    commands: &mut Commands,
    enemyassets: &ActorTextureHandles,
    event: &SpawnEnemyEvent,
) {
    commands
                        .get_entity(enemycontainer)
                        .expect("should always be atleast one entity container. if this panics we probably made more than 1")
                        .add_children(|parent| {
                            parent
                                .spawn((
                                    SkeletonBundle {
                                        name: Name::new("Skeleton"),
                                        actortype: AIEnemy::Skeleton,
                                        actorstate: MovementState {
                                            speed: 100.0,
                                            sprint_available: false,
                                            facing: FacingDirection::Idle,
                                            just_moved: false,
                                        },
                                        animation_state: AnimState {
                                            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                                            current_frames: vec![0, 1, 2, 3, 4],
                                            current_frame: 0,
                                        },
                                        available_animations: AnimationSheet {
                                            handle: enemyassets.skeleton_sheet.clone(),
                                            idle_animation: [0, 1, 2, 3, 4],
                                            down_animation: [5, 6, 7, 8, 9],
                                            up_animation: [10, 11, 12, 13, 14],
                                            right_animation: [15, 16, 17, 18, 19],
                                        },
                                        sprite: TextureAtlasSprite {
                                            custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
                                            ..default()
                                        },
                                        texture_atlas: enemyassets.skeleton_sheet.clone(),
                                        rigidbody: RigidBodyBundle {
                                            rigidbody: bevy_rapier2d::prelude::RigidBody::Dynamic,
                                            velocity: Velocity::zero(),
                                            friction: Friction::coefficient(0.7),
                                            howbouncy: Restitution::coefficient(0.3),
                                            massprop: ColliderMassProperties::Density(0.3),
                                            rotationlocks: LockedAxes::ROTATION_LOCKED,
                                            dampingprop: Damping {
                                                linear_damping: 1.0,
                                                angular_damping: 1.0,
                                            },
                                        },
                                        brain: StupidAiBundle {
                                            actortype: ActorType(TypeEnum::Enemy),
                                            aggrodistance: AICanChase { aggro_distance: 200.0 },
                                            canmeander: AICanWander { wander_target: None, spawn_position: Some(event.spawn_position) },
                                            aiattacktimer: AIAttackTimer {
                                                timer: Timer::from_seconds(
                                                    9.5,
                                                    TimerMode::Repeating,
                                                ),
                                                is_attacking: false,
                                                is_near: false,
                                            },
                                            thinker: Thinker::build()
                                                .picker(big_brain::pickers::Highest)
                                                .when(AggroScore, AIChaseAction)
                                                .otherwise(AIWanderAction),
                                        },
                                        spatial: SpatialBundle {
                                            transform: Transform {
                                                translation: event.spawn_position,
                                                rotation: Quat::default(),
                                                scale: Vec3::ONE,
                                            },
                                            ..default()
                                        },
                                    },
                                ))
                                .with_children(|child| {
                                    child.spawn(ActorColliderBundle {
                                        name: Name::new("SkeletonCollider"),
                                        transformbundle: TransformBundle {
                                            local: (
                                                Transform {
                                                translation: (Vec3 {
                                                    x: 0.,
                                                    y: -5.,
                                                    z: ACTOR_PHYSICS_LAYER,
                                            }),
                                                ..default()
                                            }),
                                            ..default()
                                        },
                                        collider: Collider::capsule_y(10.4, 13.12),
                                    });
                                });
                        });
}

pub fn spawn_slime(
    enemycontainer: Entity,
    commands: &mut Commands,
    enemyassets: &ActorTextureHandles,
    event: &SpawnEnemyEvent,
) {
    commands
                        .get_entity(enemycontainer)
                        .expect("should always be atleast one entity container. if this panics we probably made more than 1")
                        .add_children(|parent| {
                            parent
                                .spawn((
                                    SlimeBundle {
                                        name: Name::new("Slime"),
                                        actortype: AIEnemy::Slime,
                                        actorstate: MovementState {
                                            speed: 50.0,
                                            sprint_available: false,
                                            facing: FacingDirection::Idle,
                                            just_moved: false,
                                        },
                                        animation_state: AnimState {
                                            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                                            current_frames: vec![0, 1, 2, 3, 4],
                                            current_frame: 0,
                                        },
                                        available_animations: AnimationSheet {
                                            handle: enemyassets.slime_sheet.clone(),
                                            idle_animation: [0, 1, 2, 3, 4],
                                            down_animation: [5, 6, 7, 8, 9],
                                            up_animation: [10, 11, 12, 13, 14],
                                            right_animation: [15, 16, 17, 18, 19],
                                        },
                                        sprite: TextureAtlasSprite {
                                            custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
                                            ..default()
                                        },
                                        texture_atlas: enemyassets.slime_sheet.clone(),
                                        rigidbody: RigidBodyBundle {
                                            rigidbody: bevy_rapier2d::prelude::RigidBody::Dynamic,
                                            velocity: Velocity::zero(),
                                            friction: Friction::coefficient(0.7),
                                            howbouncy: Restitution::coefficient(1.3),
                                            massprop: ColliderMassProperties::Density(0.6),
                                            rotationlocks: LockedAxes::ROTATION_LOCKED,
                                            dampingprop: Damping {
                                                linear_damping: 1.0,
                                                angular_damping: 1.0,
                                            },
                                        },
                                        brain: StupidAiBundle {
                                            actortype: ActorType(TypeEnum::Enemy),
                                            aggrodistance: AICanChase { aggro_distance: 200.0 },
                                            canmeander: AICanWander { wander_target: None, spawn_position: Some(event.spawn_position) },
                                            aiattacktimer: AIAttackTimer {
                                                timer: Timer::from_seconds(
                                                    9.5,
                                                    TimerMode::Repeating,
                                                ),
                                                is_attacking: false,
                                                is_near: false,
                                            },
                                            thinker: Thinker::build()
                                                .picker(big_brain::pickers::Highest)
                                                .when(AggroScore, AIChaseAction)
                                                .otherwise(AIWanderAction),
                                        },
                                        spatial: SpatialBundle {
                                            transform: Transform {
                                                translation: event.spawn_position,
                                                rotation: Quat::default(),
                                                scale: Vec3::ONE,
                                            },
                                            ..default()
                                        },
                                    },
                                ))
                                .with_children(|child| {
                                    child.spawn(ActorColliderBundle {
                                        name: Name::new("SlimeCollider"),
                                        transformbundle: TransformBundle {
                                            local: (
                                                Transform {
                                                translation: (Vec3 {
                                                x: 0.,
                                                    y: -5.,
                                                    z: ACTOR_PHYSICS_LAYER,
                                            }),
                                                ..default()
                                            }),
                                            ..default()
                                        },
                                        collider: Collider::capsule(Vec2::new(0.0, -10.6), Vec2::new(0.0, -12.6), 16.5),
                                    });
                                });
                        });
}
