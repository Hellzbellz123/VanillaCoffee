use bevy::prelude::*;
use bevy_aseprite_ultra::{
    prelude::{Animation, AnimationState, Aseprite},
    NotLoaded,
};
use bevy_rapier2d::prelude::*;

use crate::{
    game::{
        attributes_stats::{CharacterStatBundle, EquipmentStats, ProjectileStats},
        characters::{
            ai::components::AiType,
            components::{CharacterMoveState, CharacterType},
        },
        components::{ActorColliderType, TimeToLive},
        items::weapons::components::{AttackDamage, WeaponDescriptor, WeaponHolder},
    },
    loading::registry::RegistryIdentifier,
};

/// bundle used too spawn "actors"
#[derive(Bundle, Reflect, Clone)]
pub struct CharacterBundle {
    /// actor name
    pub name: Name,
    /// id too get actor definition
    pub identifier: RegistryIdentifier,
    /// actors current movement data
    pub move_state: CharacterMoveState,
    /// actor type
    pub actor_type: CharacterType,
    /// actor stat
    pub stats: CharacterStatBundle,
    /// is character ai controlled or player controlled
    pub controller: AiType,
    #[reflect(ignore)]
    /// required components too render an Aseprite file as an Actor
    pub render: Aspen2dRenderBundle,
    #[reflect(ignore)]
    /// actor collisions and movement
    pub physics: Aspen2dPhysicsBundle,
}

/// bundle for spawning weapons
#[derive(Bundle, Reflect, Clone)]
pub struct WeaponBundle {
    /// weapon name
    pub name: Name,
    /// accesor for weapon definition
    pub identifier: RegistryIdentifier,
    /// who is holding this weapon, and what slot it is in
    pub holder: WeaponHolder,
    /// weapons damage when used
    pub damage: AttackDamage,
    /// how this weapon applies its damage, along with config
    pub weapon_type: WeaponDescriptor,
    /// stats applied too holder
    pub stats: EquipmentStats,
    #[reflect(ignore)]
    /// requirements too render weapon
    pub render: Aspen2dRenderBundle,
    #[reflect(ignore)]
    /// weapon physics
    pub physics: Aspen2dPhysicsBundle,
}

/// bundle too spawn projectiles
#[derive(Bundle)]
pub struct ProjectileBundle {
    /// projectile name
    pub name: Name,
    /// projectile stats
    pub projectile_stats: ProjectileStats,
    /// projectile lifetime
    pub ttl: TimeToLive,
    /// projectile Sprite
    pub sprite_bundle: SpriteBundle,
    /// projectile collisions and movement
    pub rigidbody_bundle: Aspen2dPhysicsBundle,
}

/// The `AspenRenderBundle` holds all components needed to render Aseprite files as Actors.
#[derive(Bundle, Default, Clone)]
pub struct Aspen2dRenderBundle {
    /// asperite asset for this sprite
    pub handle: Handle<Aseprite>,
    /// animation controller
    pub animation: Animation,
    /// animation play information
    pub animation_state: AnimationState,
    /// marks not yet loaded sprite entity
    pub not_loaded: NotLoaded,
    /// texture atlas for final sprite image
    pub atlas: TextureAtlas,
    /// sprite configuration
    pub sprite: Sprite,
}

// TODO: upstream reflect/debug fixes and remove this manual impl
impl std::fmt::Debug for Aspen2dRenderBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Aspen2dRenderBundle")
            .field("sprite asset", &self.handle)
            .field("animation name", &self.animation.tag)
            .field("sprite cfg", &self.sprite)
            .finish_non_exhaustive()
    }
}

/// tags this entity for collider creation
/// requires that entities have an Aabb for proper collider size
#[derive(Debug, Clone, Component)]
pub struct NeedsCollider;

/// collider bundle for actors
#[derive(Debug, Bundle)]
pub struct AspenColliderBundle {
    /// name of collider
    pub name: Name,
    /// type of collider
    pub tag: ActorColliderType,
    /// collider shape
    pub collider: NeedsCollider,
    /// collision groups
    pub collision_groups: CollisionGroups,
    /// collider transform
    pub transform_bundle: TransformBundle,
}

/// bundle for collisions and movement
/// REQUIRES child collider too work properly
#[derive(Bundle, Clone)]
pub struct Aspen2dPhysicsBundle {
    /// rigidbody
    pub rigidbody: RigidBody,
    /// velocity
    pub velocity: Velocity,
    /// friction
    pub friction: Friction,
    /// bounciness
    pub how_bouncy: Restitution,
    /// `RigidBody` Mass
    pub mass_prop: ColliderMassProperties,
    /// rotation locks
    pub rotation_locks: LockedAxes,
    /// velocity damping
    pub damping_prop: Damping,
}

impl Aspen2dPhysicsBundle {
    /// default enemy rigidbody stats
    pub const DEFAULT_CHARACTER: Self = Self {
        rigidbody: bevy_rapier2d::prelude::RigidBody::Dynamic,
        velocity: Velocity::zero(),
        friction: Friction::coefficient(0.7),
        how_bouncy: Restitution::coefficient(0.3),
        mass_prop: ColliderMassProperties::Density(0.3),
        rotation_locks: LockedAxes::ROTATION_LOCKED,
        damping_prop: Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        },
    };
}

impl Default for Aspen2dPhysicsBundle {
    fn default() -> Self {
        Self::DEFAULT_CHARACTER
    }
}

impl std::fmt::Debug for WeaponBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeaponBundle")
            .field("name", &self.name)
            .field("identifier", &self.identifier)
            .field("holder", &self.holder)
            .field("damage", &self.damage)
            .field("weapon_type", &self.weapon_type)
            .field("stats", &self.stats)
            .field("render", &self.render)
            .field("rigidbody_bundle", &self.physics.rigidbody)
            .finish()
    }
}
