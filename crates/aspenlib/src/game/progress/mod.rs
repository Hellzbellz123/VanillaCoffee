// switch too hideout if player dies inside dungeon
// switch too hideout if player exits dungeon by choice
// lock doors of room player is currently in until room enemies are defeated
// if player defeats boss, regenerate dungeon and bump dungeon level

use crate::{
    game::game_world::dungeonator_v2::components::BossState, register_types, AppStage, GameStage,
};
use bevy::prelude::*;

mod dungeon_tracking;

/// player progression tracking module
pub struct GameProgressPlugin;

impl Plugin for GameProgressPlugin {
    fn build(&self, app: &mut App) {
        register_types!(app, [ProgressManager, CurrentRunInformation, PlayerSaveInformation]);

        app.insert_resource(CurrentRunInformation::default())
        .insert_resource(PlayerSaveInformation::default());

        app.add_systems(OnExit(AppStage::Loading), spawn_progress_manager);
        app.add_systems(
            FixedUpdate,
            (
                dungeon_tracking::update_boss_state,
                dungeon_tracking::update_player_current_room,
            )
                .run_if(in_state(GameStage::PlayingGame)),
        );
    }
}

/// player progression tracker
#[derive(Debug, Reflect, Component, Clone)]
#[reflect(Component)]
pub struct ProgressManager {
    /// player progress in CURRENT dungeon
    current: CurrentDungeonState,
    /// player progress unrelated too CURRENT dungeon
    overall: OverallProgressState,
}

/// current dungeon progression for player
#[derive(Debug, Reflect, Component, Clone)]
pub struct CurrentDungeonState {
    /// boss combat state
    boss_state: BossState,
    /// current room entity id
    current_room: Option<Entity>,
    /// boss entity id
    boss_id: Option<Entity>,
}

/// overall progress for player
#[derive(Debug, Reflect, Component, Clone)]
pub struct OverallProgressState {
    /// how much coin player has earned
    coin: i32,
    /// how much xp player has earnend
    xp: i32,
    /// how many enemies player has defeated
    kills: i32,
}

/// information tracked for current run
#[derive(Debug, Clone, Copy, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct CurrentRunInformation {
    /// damage dealt by player this run
    pub enemy_physical_damage_taken: f32,
    /// damage dealt too player this run
    pub player_physical_damage_taken: f32,
    /// enemies killed by player this run
    pub enemies_deaths: i32,
    /// times player has died
    pub player_deaths: i32,
    /// amount of damage enemy's have fired that hit player and didn't get counted
    pub enemy_damage_sent: f32,
    /// amount of damage player have fired that hit enemy and didn't get counted
    pub player_damage_sent: f32,
}

//TODO: save this too file, load from file when rebooting game
/// information tracked for player save state
#[derive(Debug, Clone, Copy, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerSaveInformation {
    /// damage player has cause with this save
    pub all_time_damage: f32,
    /// amount of times player has finishes a run
    pub runs_completed: i32,
    /// amount of times play has started a run
    pub runs_started: i32,
    /// amount of money player has earned
    pub player_money: i32,
    /// total amount of player deaths
    pub total_deaths: i32,
    /// total amonut of items player has collected
    pub items_got: i32,
}

/// creates entity for tracking player progress inside dungeon
fn spawn_progress_manager(mut cmds: Commands) {
    // load character save state here?

    cmds.spawn((
        Name::new("ProgressManager"),
        ProgressManager {
            current: CurrentDungeonState {
                boss_state: BossState::UnSpawned,
                current_room: None,
                boss_id: None,
            },
            overall: OverallProgressState {
                coin: 0,
                xp: 0,
                kills: 0,
            },
        },
    ));
}
