use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::action_state::ActionState;

use crate::{
    game::input::{action_maps, AspenInputSystemSet},
    loading::splashscreen::MainCamera,
    AppState,
};

// TODO: merge software cursor and actor_targeting modules and move input module outside game module
// create target circle, change circle based on AspenCursorPosition.world and if hitting character
// if object is 'interactable' change color of target cursor and show 'press e to interact'

pub struct ActorTargetingPlugin;

impl Plugin for ActorTargetingPlugin {
    fn build(&self, app: &mut App) {
        // TODO: brainstorm actor targeting system
    }
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct AspenTargetingReticle {
    current_target: Option<Entity>,
}
