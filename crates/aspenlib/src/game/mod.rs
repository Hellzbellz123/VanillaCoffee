use bevy::{prelude::*, render::primitives::Aabb};
use bevy_rapier2d::prelude::Collider;

use crate::{
    bundles::NeedsCollider,
    game::components::{ActorColliderType, TimeToLive},
    loading::{
        custom_assets::actor_definitions::{CharacterDefinition, ItemDefinition},
        registry::RegistryIdentifier,
    },
    playing_game, register_types,
    utilities::{scale_to_fit, EntityCreator},
};

/// animation functionality
pub mod animations;
/// character/item stats functionality
pub mod attributes_stats;
/// audio data for game
pub mod audio;
/// game characters spawning and functionality
pub mod characters;
/// combat functionality plugin
pub mod combat;
/// shared components for game
pub mod components;
/// sanctuary and dungeon generator
pub mod game_world;
/// input from player
pub mod input;
/// Game `UserInterface` Module, contains interface plugin
pub mod interface;
/// game item spawning and functionality
pub mod items;
/// player progression module
pub mod progress;

/// are we in dungeon yet?
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default, Reflect)]
pub enum GameProgress {
    /// homeroom
    #[default]
    Sanctuary,
    /// in dungeon now
    Dungeon,
}

/// each dungeon run has 4 stages that get progressivly larger/harder
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DungeonFloor {
    /// easiest level, start here
    #[default]
    One,
    /// slighlty deeper, bit larger, more creeps
    Two,
    ///
    Three,
    /// final level of the dungeon
    Four,
}

/// plugin that holds all game functionality as plugin modules
pub struct AspenHallsPlugin;

impl Plugin for AspenHallsPlugin {
    fn build(&self, app: &mut App) {
        register_types!(app, [ActorColliderType, EntityCreator]);

        app
            // actual game plugin
            .add_plugins((
                progress::GameProgressPlugin,
                audio::AudioPlugin,
                combat::CombatPlugin,
                characters::CharactersPlugin,
                items::ItemsPlugin,
                input::InputPlugin,
                game_world::GameWorldPlugin,
                interface::InterfacePlugin,
                animations::AnimationsPlugin,
            ))
            .add_systems(
                Update,
                ((
                    // update_actor_size,
                    time_to_live
                )
                    .run_if(playing_game()),),
            )
            .add_systems(
                PreUpdate,
                add_aabb_based_colliders.run_if(any_with_component::<NeedsCollider>),
            );
    }
}

/// despawn any entity with `TimeToLive` timer thats finished
fn time_to_live(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimeToLive)>,
) {
    for (entity, mut timer) in &mut query {
        if timer.tick(time.delta()).finished() {
            let Some(a) = commands.get_entity(entity) else {
                continue;
            };
            a.despawn_recursive();
        }
    }
}

fn add_aabb_based_colliders(
    mut cmds: Commands,
    needscollider_q: Query<(Entity, &Parent), (Without<Collider>, With<NeedsCollider>)>,
    aabbs_q: Query<&Aabb>,
    // names_q: Query<&Name>,
) {
    for (needs_collider, parent) in &needscollider_q {
        let Ok(aabb) = aabbs_q.get(parent.get()) else {
            continue;
        };
        let start = Vec2::ZERO
            + Vec2 {
                x: 0.0,
                y: aabb.half_extents.x / 2.0,
            };
        let end = Vec2::ZERO
            + Vec2 {
                x: 0.0,
                y: aabb.half_extents.y,
            };

        let collider = Collider::capsule(start, end, aabb.half_extents.x / 2.0);
        cmds.entity(needs_collider)
            .remove::<NeedsCollider>()
            .insert(collider);
    }
}

/// modifys all actors in game such that the actors image asset is rendered relative too pixel size
fn update_actor_size(
    mut query: Query<(&mut Sprite, &TextureAtlas, &RegistryIdentifier)>,
    texture_atlasses: Res<Assets<TextureAtlasLayout>>,
    obje_assets: Res<Assets<ItemDefinition>>,
    char_assets: Res<Assets<CharacterDefinition>>,
) {
    for (mut sprite, texture_atlas, registry_identifier) in &mut query {
        if sprite.custom_size.is_some() {
            continue;
        }

        let Some(atlas) = texture_atlasses.get(texture_atlas.layout.id()) else {
            error!("texture atlas layout for this spritesheet is missing");
            continue;
        };
        let original_size = atlas.textures.first().expect("no textures in atlas").size();

        let final_size: Vec2 = {
            let maybe_characer = char_assets
                .iter()
                .find(|(_, asset)| asset.actor.identifier == *registry_identifier);
            let maybe_item = obje_assets
                .iter()
                .find(|(_, asset)| asset.actor.identifier == *registry_identifier);

            if let Some((_, def)) = maybe_characer {
                Vec2::splat(def.actor.tile_size)
            } else if let Some((_, def)) = maybe_item {
                Vec2::splat(def.actor.tile_size)
            } else {
                warn!("character has no asset");
                continue;
            }
        };

        let new_custom_size = scale_to_fit(original_size.as_vec2(), final_size);
        sprite.custom_size = Some(new_custom_size);
    }
}
