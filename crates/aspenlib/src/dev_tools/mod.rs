use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use crate::{
    dev_tools::debug_visuals::{DebugDraw, DrawnMap},
    game::input::action_maps::Gameplay,
    register_types, GeneralSettings,
};
use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};
use bevy_ecs_ldtk::{prelude::LdtkProject, GridCoords, IntGridCell, LayerMetadata};
use bevy_mod_debugdump::{render_graph, schedule_graph, schedule_graph_dot};
use bevy_rapier2d::prelude::{DebugRenderContext, RapierDebugRenderPlugin};
use big_brain::{
    choices::Choice,
    prelude::{Actor, HasThinker, Score, Scorer, Thinker},
};
use leafwing_input_manager::prelude::ActionState;

#[cfg(not(any(
    target_os = "ios",
    target_os = "android",
    target_family = "wasm",
    not(feature = "develop")
)))]
pub mod debug_dirs;

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
                debug: true,
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
        debug_dirs::dump_launch_directory();

        #[cfg(not(any(
            target_os = "ios",
            target_os = "android",
            target_family = "wasm",
            not(feature = "develop")
        )))]
        debug_dump_graphs(app);
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

// /// debug plugin for vanillacoffee
// /// holds type registration, diagnostics, and inspector stuff
// pub mod debug_plugin {
//     use bevy::{
//         diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
//         input::common_conditions::input_toggle_active,
//         prelude::*,
//     };
//     // use bevy_debug_text_overlay::OverlayPlugin;
//     use bevy_ecs_ldtk::{assets::LdtkProject, GridCoords, IntGridCell, LayerMetadata};
//     use bevy_inspector_egui::{
//         bevy_inspector::{ui_for_all_assets, ui_for_resources, ui_for_world_entities_filtered},
//         quick::StateInspectorPlugin,
//     };
//     use bevy_mod_debugdump::{render_graph, render_graph_dot, schedule_graph, schedule_graph_dot};
//     use bevy_prototype_lyon as svg;
//     use bevy_rapier2d::render::RapierDebugRenderPlugin;
//     use big_brain::{
//         choices::Choice,
//         prelude::{Actor, HasThinker, Score, Scorer, Thinker},
//     };

//     // #[cfg(feature = "develop")]
//     // #[cfg(not(any(target_os = "android", target_family = "wasm")))]
//     // use crate::dev_tools::debug_dirs::debug_directory;

//     use crate::{
//         game::{
//             game_world::{components::CharacterSpawner, dungeonator_v2::GeneratorState},
//             items::weapons::components::{AttackDamage, CurrentlyDrawnWeapon},
//         },
//         register_types, AppStage, GameStage,
//     };

//     use std::{
//         fs,
//         path::{Path, PathBuf},
//         time::Duration,
//     };

//     /// debug plugin for Aspen Halls.
//     /// registers types from plugins and the game, prints diagnostics too the console, and spawns an `world` inspector and a `state` inspector
//     pub struct DebugPlugin;

//     impl Plugin for DebugPlugin {
//         fn build(&self, app: &mut App) {
//             #[cfg(not(any(target_os = "android", target_family = "wasm")))]
//             debug_directory();

//             // add other debug plugins
//             .add_plugins((
//                 #[cfg(feature = "develop")]
//                 RapierDebugRenderPlugin::default(),
//                 FrameTimeDiagnosticsPlugin,
//                 LogDiagnosticsPlugin {
//                     wait_duration: Duration::from_secs(20),
//                     ..Default::default()
//                 },
//                 // OverlayPlugin {
//                 //     font_size: 32.0,
//                 //     ..Default::default()
//                 // },
//             ))
//             // TODO: refactor these systems into nice sets and stages
//             .add_systems(
//                 Update,
//                 (
//                     // (debug_visualize_spawner, debug_visualize_weapon_spawn_point)
//                     //     .run_if(in_state(AppStage::Running)),
//                     world_inspector_ui.run_if(input_toggle_active(
//                         if cfg!(feature = "develop") {
//                             true
//                         } else {
//                             false
//                         },
//                         KeyCode::F3,
//                     )),
//                 ),
//             );

//             debug_dump_graphs(app);
//         }
//     }

/// dumps scheduling graphs for given App
pub fn debug_dump_graphs(app: &mut App) {
    let target = Path::new(".schedule");
    match target.try_exists() {
        Err(error) => {
            warn!("problem with {:?} directory: {}", target, error);
        }
        Ok(exists) => {
            if !exists {
                warn!(
                    "Not dumping schedules because {:?} directory does not exist",
                    target
                );
                warn!(
                    "Create {:?} directory in cwd too dump schedule graphs",
                    target
                );
                return;
            }
            warn!("Dumping graphs");

            let schedule_theme = schedule_graph::settings::Style::dark_github();
            let render_theme = render_graph::settings::Style::dark_github();

            let settings = schedule_graph::Settings {
                ambiguity_enable: false,
                ambiguity_enable_on_world: false,
                style: schedule_theme,
                collapse_single_system_sets: true,
                // prettify_system_names: true,
                ..Default::default()
            };

            let render_graph_settings = render_graph::Settings {
                style: render_theme,
            };

            let pre_startup_graph = schedule_graph_dot(app, PreStartup, &settings);
            let main_startup_graph = schedule_graph_dot(app, Startup, &settings);
            let post_startup_graph = schedule_graph_dot(app, PostStartup, &settings);
            let first_schedule = schedule_graph_dot(app, First, &settings);
            let pre_update_schedule = schedule_graph_dot(app, PreUpdate, &settings);
            let main_update_schedule = schedule_graph_dot(app, Update, &settings);
            let post_update_schedule = schedule_graph_dot(app, PostUpdate, &settings);
            let last_schedule = schedule_graph_dot(app, Last, &settings);

            // let render_graph = render_graph_dot(app, &render_graph_settings);

            write_graphs(
                target.to_path_buf(),
                (
                    pre_startup_graph,
                    main_startup_graph,
                    post_startup_graph,
                    first_schedule,
                    pre_update_schedule,
                    main_update_schedule,
                    post_update_schedule,
                    last_schedule,
                    // render_graph,
                ),
            );
        }
    }
}

/// dumps schedule as a graph
fn write_graphs(
    folder: PathBuf,
    dotfiles: (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        // String,
    ),
) {
    let (
        pre_startup_graph,
        main_startup_graph,
        post_startup_graph,
        first_schedule,
        pre_update_schedule,
        main_update_schedule,
        post_update_schedule,
        last_schedule,
        // render_graph,
    ) = dotfiles;

    match fs::write(folder.join("0-pre_startup_schedule.dot"), pre_startup_graph) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(
        folder.join("1-main_startup_schedule.dot"),
        main_startup_graph,
    ) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(folder.join("2-post_startup_graph.dot"), post_startup_graph) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(folder.join("3-first_schedule.dot"), first_schedule) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(
        folder.join("4-pre_update_schedule.dot"),
        pre_update_schedule,
    ) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(
        folder.join("5-main_update_schedule.dot"),
        main_update_schedule,
    ) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(
        folder.join("6-post_update_schedule.dot"),
        post_update_schedule,
    ) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(folder.join("7-last_schedule.dot"), last_schedule) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }

    // match fs::write(folder.join("Z-render_graph.dot"), render_graph) {
    //     Ok(()) => {}
    //     Err(e) => warn!("{}", e),
    // }
}
