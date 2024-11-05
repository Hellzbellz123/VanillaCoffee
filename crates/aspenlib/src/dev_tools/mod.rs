use std::time::Duration;

use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};
use bevy_ecs_ldtk::{prelude::LdtkProject, GridCoords, IntGridCell, LayerMetadata};
use bevy_rapier2d::prelude::{DebugRenderContext, RapierDebugRenderPlugin};
use big_brain::{
    choices::Choice,
    prelude::{Actor, HasThinker, Score, Scorer, Thinker},
};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    dev_tools::debug_visuals::{DebugDraw, DrawnMap},
    game::input::action_maps::Gameplay,
    register_types, GeneralSettings,
};

#[cfg(not(any(
    target_os = "ios",
    target_os = "android",
    target_family = "wasm",
    not(feature = "develop")
)))]
pub mod debug_dirs;

#[cfg(not(any(
    target_os = "ios",
    target_os = "android",
    target_family = "wasm",
    not(feature = "develop")
)))]
pub mod dump_schedules;

pub mod console;
pub mod debug_visuals;
pub mod egui_tools;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct DebugConfig {
    enabled: bool,
    pub drawmap: DrawnMap,
    pub show_world_inspector: bool,
    pub disable_animation: bool,
    pub physics_draw: bool,
    pub aabb_draw: bool,
    pub show_appstate: bool,
    pub show_gamestate: bool,
    pub show_generatorstate: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            drawmap: DrawnMap::default(),
            enabled: cfg!(feature = "develop"),
            show_world_inspector: cfg!(feature = "develop"),
            physics_draw: cfg!(feature = "develop"),
            disable_animation: false,
            aabb_draw: false,
            show_appstate: false,
            show_gamestate: false,
            show_generatorstate: false,
        }
    }
}

pub struct AspenDevToolsPlugin;

impl Plugin for AspenDevToolsPlugin {
    fn build(&self, app: &mut App) {
        // debug tools unregistered types
        register_types!(app, [DrawnMap, DebugDraw, DebugConfig]);
        // ldtk unregistered types
        register_types!(
            app,
            [
            LdtkProject,
            LayerMetadata,
            IntGridCell,
            GridCoords,
            Handle<LdtkProject>
            ]
        );
        // BigBrain unregistered types
        register_types!(
            app,
            [
                Actor,
                big_brain::prelude::Action,
                Scorer,
                Score,
                Choice,
                Thinker,
                HasThinker
            ]
        );

        app.init_resource::<DebugConfig>();

        app.add_plugins((
            // internal tools
            console::QuakeConPlugin,
            debug_visuals::DebugVisualsPlugin,
            egui_tools::EguiToolsPlugin,
            // external tools
            RapierDebugRenderPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            LogDiagnosticsPlugin {
                debug: false,
                wait_duration: Duration::from_secs(5),
                filter: None,
            },
        ));

        app.add_systems(
            Update,
            (
                toggle_debug_systems,
                toggle_physics_visualizations,
                bridge_debug_cfg_setting.run_if(resource_changed::<GeneralSettings>),
            ),
        );

        #[cfg(not(any(
            target_os = "ios",
            target_os = "android",
            target_family = "wasm",
            not(feature = "develop")
        )))]
        {
            debug_dirs::dump_launch_directory();
            dump_schedules::debug_dump_graphs(app);
        }
    }
}

fn bridge_debug_cfg_setting(
    mut debug_ctrl: ResMut<DebugConfig>,
    general_settings: ResMut<GeneralSettings>,
) {
    if general_settings.is_changed() || debug_ctrl.enabled != general_settings.enable_debug {
        debug_ctrl.enabled = general_settings.enable_debug;
    }
}

fn toggle_debug_systems(
    mut cfg: ResMut<GeneralSettings>,
    mut debug_ctrl: ResMut<DebugConfig>,
    input: Res<ActionState<Gameplay>>,
) {
    if input.just_pressed(&Gameplay::DebugF3) {
        if cfg.enable_debug {
            debug_ctrl.enabled = false;
            cfg.enable_debug = false;
        } else {
            debug_ctrl.enabled = true;
            cfg.enable_debug = true;
        }
    }
}

fn toggle_physics_visualizations(
    debug_ctrl: Res<DebugConfig>,
    mut physics_debug: ResMut<DebugRenderContext>,
) {
    if physics_debug.enabled != debug_ctrl.physics_draw {
        physics_debug.enabled = debug_ctrl.physics_draw;
    }
}
