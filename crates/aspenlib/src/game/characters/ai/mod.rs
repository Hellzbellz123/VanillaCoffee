use bevy::prelude::*;
use big_brain::{prelude::*, BigBrainPlugin};
use std::{collections::VecDeque, time::Duration};

use crate::{
    game::characters::ai::{
            components::{
                AIAutoShootConfig, AIChaseAction, AICombatAggroConfig, AIShootAction,
                AIWanderAction, AIWanderConfig, AiType, AttackScorer, ChaseScorer,
            },
            skillsusing_ai::{
                AIPatternEnergy, AIShootPatternsConfig, PatternCost, ShootPattern,
                SkillusingAiPlugin, MAX_PATTERN_ENERGY,
            },
        },
    register_types,
};


// TODO: get actor ai values from definition and use them when spawning/inserting actors ai

// move ai config too `AiType` enum, each ai type gets a scorer that
// determines its actions using config data held inside AiType,
// AiType is defined inside character_definition

// merge AiCombatAggroConfig and AiAutoShoot config into a AiBasicCombatConfig
// move pattern related code into combat module and make all combat event based.

// basic combat should have 3 actions
// chase, attack, flee
// if basiccombat fails then we should go back too wander action



use self::stupid_ai::StupidAiPlugin;

/// ai components
pub mod components;
/// stupid ai stuff
pub mod stupid_ai;

/// skll using ai type for bosses and others
pub mod skillsusing_ai;

/// handles different AI classes
pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        register_types!(
            app,
            [
                Actor,
                ActionState,
                Thinker,
                Score,
                Action,
                ChaseScorer,
                AttackScorer,
                AICombatAggroConfig,
                AIWanderConfig,
                AIAutoShootConfig,
                AIChaseAction,
                AIWanderAction,
                AIShootAction
            ]
        );

        app.add_plugins((
            BigBrainPlugin::new(Update),
            StupidAiPlugin,
            SkillusingAiPlugin,
        ))
        .add_systems(Update, initialize_ai);
    }
}

/// finds all characters wanting ai and adds required ai components
#[allow(clippy::type_complexity)]
fn initialize_ai(
    mut commands: Commands,
    ai_controlled: Query<
        (Entity, &AiType, &GlobalTransform),
        Added<AiType>,
    >,
) {
    for (character, wanted_ai, pos) in &ai_controlled {
        match wanted_ai {
            AiType::Stupid => {
                insert_stupid_ai(&mut commands, character, pos);
            }
            AiType::Boss => {
                insert_skillusing_ai(&mut commands, character, pos);
                error!("boss ai is not finished");
            }
            AiType::Critter => error!("ai type not implemented"),
            AiType::PlayerPet => error!("ai type not implemented"),
            AiType::FollowerHero => error!("ai type not implemented"),
        }

        // commands.entity(character).remove::<AiType>();
    }
}

/// inserts required components for basic AI functionality
fn insert_stupid_ai(commands: &mut Commands<'_, '_>, character: Entity, pos: &GlobalTransform) {
    commands.entity(character).insert(stupid_ai::BasicAiBundle {
        combat_config: AICombatAggroConfig {
            chase_start: 6,
            chase_end: 13,
            shoot_range: 8,
            personal_space: 2,
            runaway_hp: 20.0,
        },
        wander_config: AIWanderConfig {
            wander_target: None,
            spawn_position: Some(pos.translation().truncate()),
            wander_distance: 7,
            idle_timer: Timer::from_seconds(2.0, TimerMode::Once),
        },
        shoot_config: AIAutoShootConfig {
            find_target_range: 8,
            timer: Timer::new(Duration::from_secs_f32(0.2), TimerMode::Once),
            should_shoot: false,
            can_shoot: false,
        },
        // thinker builder should be built differently
        thinker: Thinker::build()
            .picker(big_brain::pickers::Highest)
            .when(ChaseScorer, AIChaseAction)
            .when(AttackScorer, AIShootAction)
            .otherwise(AIWanderAction),
    });
}

/// default bullet spawn pattern
fn basic_bullet_pattern() -> VecDeque<(PatternCost, ShootPattern)> {
    let mut map = VecDeque::new();
    map.push_front((
        PatternCost(40),
        ShootPattern::BulletsOverArc {
            arc: 360,
            amount: 16,
            waves: 14,
            rotation_per_wave: 5,
        },
    ));
    map.push_front((
        PatternCost(40),
        ShootPattern::BulletsOverArc {
            arc: 360,
            amount: 12,
            waves: 14,
            rotation_per_wave: 15,
        },
    ));
    map.push_front((
        PatternCost(10),
        ShootPattern::BulletsOverArc {
            arc: 45,
            amount: 8,
            waves: 16,
            rotation_per_wave: 15,
        },
    ));

    map
}

/// add skill using ai components too required entity
fn insert_skillusing_ai(commands: &mut Commands<'_, '_>, character: Entity, pos: &GlobalTransform) {
    commands.entity(character).insert((
        AIPatternEnergy {
            per_second: 5.0,
            current: MAX_PATTERN_ENERGY,
        },
        skillsusing_ai::SkillusingAIBundle {
            shootpattern: AIShootPatternsConfig {
                patterns: basic_bullet_pattern(),
                time_between_patterns: Timer::from_seconds(2.0, TimerMode::Repeating),
            },
            combat_config: AICombatAggroConfig {
                chase_start: 10,
                chase_end: 16,
                shoot_range: 8,
                personal_space: 3,
                runaway_hp: 20.0,
            },
            shoot_config: AIAutoShootConfig {
                find_target_range: 8,
                timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once),
                should_shoot: false,
                can_shoot: false,
            },
            wander_config: AIWanderConfig {
                wander_target: None,
                spawn_position: Some(pos.translation().truncate()),
                wander_distance: 8,
                idle_timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
            thinker: Thinker::build()
                .picker(big_brain::pickers::Highest)
                .when(ChaseScorer, AIChaseAction)
                .when(AttackScorer, AIShootAction)
                .otherwise(AIWanderAction),
        },
    ));
}
