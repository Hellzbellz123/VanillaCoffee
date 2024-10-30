use bevy::{
    ecs::{schedule::Condition, system::Res},
    log::{error, info},
    math::Vec2,
    prelude::{
        in_state, on_event, Assets, Commands, DespawnRecursiveExt, Entity, EventReader,
        GlobalTransform, IntoSystemConfigs, OnEnter, OrthographicProjection, Parent, Plugin, Query,
        SpatialBundle, Transform, Update, With, Without,
    },
};
use bevy_ecs_ldtk::{
    prelude::{LdtkExternalLevel, LevelEvent, LevelSet},
    LevelIid, LevelSelection,
};
use bevy_mod_picking::{
    events::{Down, Pointer},
    prelude::{On, PickableBundle},
};
use bevy_rapier2d::prelude::CollisionEvent;
use log::warn;

use crate::{
    consts::ACTOR_Z_INDEX,
    game::{
        characters::{
            components::CharacterMoveState,
            player::{PlayerSelectedHero, SelectThisHeroForPlayer},
        },
        game_world::{
            components::HeroLocation,
            dungeonator_v2::GeneratorState,
            hideout::systems::{spawn_hideout, teleporter_collisions},
        },
        items::weapons::components::AttackDamage,
    },
    loading::{
        registry::{ActorRegistry, RegistryIdentifier},
        splashscreen::MainCamera,
    },
    AppStage,
};

use self::systems::HideoutTag;

/// hideout systems
pub mod systems;

/// plugin for safe house
pub struct HideOutPlugin;

// TODO: spawn different hideout when player beats boss
// spawn TestingHalls as first level if debug ONLY

impl Plugin for HideOutPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("registering ldtk map cells and adding teleport event");
        app.add_systems(OnEnter(AppStage::Starting), spawn_hideout);
        app.add_systems(OnEnter(GeneratorState::LayoutDungeon), despawn_hideout);
        app.add_systems(
            Update,
            (
                // TODO: fix scheduling
                teleporter_collisions.run_if(on_event::<CollisionEvent>()),
                create_playable_heroes
                    .run_if(in_state(AppStage::Running).and_then(on_event::<LevelEvent>())),
            ),
        );
    }
}

/// spawns selectable heroes at each available `HeroSpot`
fn create_playable_heroes(
    registry: Res<ActorRegistry>,
    selected_level: Res<LevelSelection>,
    level_assets: Res<Assets<LdtkExternalLevel>>,
    hero_spots: Query<&GlobalTransform, With<HeroLocation>>,
    mut level_spawn_events: EventReader<LevelEvent>,
    mut commands: Commands,
    mut already_spawned_hero: Query<
        (&RegistryIdentifier, &mut Transform),
        (With<PlayerSelectedHero>, Without<MainCamera>),
    >,
    mut camera_query: Query<
        (&mut Transform, &mut OrthographicProjection),
        (With<MainCamera>, Without<PlayerSelectedHero>),
    >,
) {
    let level = match selected_level.into_inner() {
        LevelSelection::Identifier(a) => {
            let level_asset = level_assets
                .iter()
                .find(|f| f.1.data().identifier() == a)
                .expect("msg")
                .1
                .data();
            let level_iid = level_asset.iid();
            LevelIid::new(level_iid)
        }
        LevelSelection::Iid(level_iid) => level_iid.clone(),
        LevelSelection::Uid(_) => panic!("uid grabbing for levels is unhandled as of yet"),
        LevelSelection::Indices(_) => {
            panic!("unable too handle multiple level spawning hero spawners as of yet")
        }
    };

    for event in level_spawn_events.read() {
        let existing_hero = already_spawned_hero.get_single_mut();
        if let LevelEvent::Transformed(_iid) = event {
            if _iid != &level {
                continue;
            }
            let hero_spots: Vec<&GlobalTransform> = hero_spots.iter().collect();
            if registry.characters.heroes.is_empty() {
                error!("no heroes too pick from");
            }
            if hero_spots.is_empty() {
                error!("no hero spots too put heroes");
            }

            info!("preparing heroes and focusing camera");
            let hero_spots_iter = hero_spots.iter();

            info!("placing heroes");
            populate_hero_spots(&registry, existing_hero, hero_spots_iter, &mut commands);

            adjust_camera_focus(hero_spots, &mut camera_query);
        }
    }
}

/// fills hero slots with selectable heroes
fn populate_hero_spots(
    registry: &Res<ActorRegistry>,
    existing_hero: Result<
        (&RegistryIdentifier, bevy::prelude::Mut<Transform>),
        bevy::ecs::query::QuerySingleError,
    >,
    mut hero_spots_iter: std::slice::Iter<&GlobalTransform>,
    commands: &mut Commands,
) {
    // TODO: swap this around for better expandability?
    registry
        .characters
        .heroes
        .values()
        .filter(|f| {
            if let Ok((a, _)) = existing_hero {
                *a != f.identifier
            } else {
                true
            }
        })
        .for_each(|bundle| {
            let Some(spot) = hero_spots_iter.next() else {
                error!("no more hero spots");
                return;
            };

            commands.spawn((
                bundle.clone(),
                PickableBundle::default(),
                On::<Pointer<Down>>::send_event::<SelectThisHeroForPlayer>(),
                SpatialBundle::from_transform(Transform::from_translation(
                    spot.translation().truncate().extend(ACTOR_Z_INDEX),
                )),
            ));
        });

    if existing_hero.is_ok() {
        let (_id, mut position) = existing_hero.unwrap();
        let new_spot = hero_spots_iter.next();
        if let Some(new_spot) = new_spot {
            warn!("moving existing hero too unoccupied hero spot");
            position.translation = new_spot.translation().truncate().extend(ACTOR_Z_INDEX);
        } else {
            warn!("no empty hero spot was found");
        }
    }
}

// TODO: re apply camera scale AFTER player is selected
/// modifies main camera too focus all the available hero spots
fn adjust_camera_focus(
    hero_spots: Vec<&GlobalTransform>,
    camera_query: &mut Query<
        (&mut Transform, &mut OrthographicProjection),
        (With<MainCamera>, Without<PlayerSelectedHero>),
    >,
) {
    let hero_spots_amnt = hero_spots.len() as f32;
    let sum_hero_spots: Vec2 = hero_spots.iter().map(|f| f.translation().truncate()).sum();
    let avg = sum_hero_spots / hero_spots_amnt;

    info!("focusing camera on all heroes");
    let (mut camera_pos, mut camera_proj) = camera_query.single_mut();
    camera_proj.scale = 6.0;
    camera_pos.translation = avg.extend(camera_pos.translation.z);
}

// TODO: find all uses of cmds.spawn(()) and add cleanup component
// cleanup component should be a system that querys for a specific DespawnComponent and despawns all entitys in the query
// DespawnWhenStateIs(Option<S: States/State>)
/// despawn all entities that should be cleaned up on restart
fn despawn_hideout(
    mut commands: Commands,
    characters_not_player: Query<Entity, (With<CharacterMoveState>, Without<PlayerSelectedHero>)>,
    weapons: Query<Entity, (With<AttackDamage>, Without<Parent>)>,
    hideout: Query<(Entity, &LevelSet), With<HideoutTag>>,
) {
    for (hideout, levelset) in &hideout {
        commands.entity(hideout).despawn_recursive();
    }

    for ent in &weapons {
        commands.entity(ent).despawn_recursive();
    }
    for ent in &characters_not_player {
        commands.entity(ent).despawn_recursive();
    }
}
