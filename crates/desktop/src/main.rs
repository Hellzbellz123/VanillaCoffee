#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // disable console on windows for release builds
#![doc = r"
    Aspen Halls native launcher, deals with loading the configuration.
    After valid configuration is found/created, starts bevy app
"]

use aspenlib::{
    save_load::save_settings, AudioSettings, ConfigFile, GeneralSettings, RenderSettings,
    VolumeConfig, WindowSettings,
};
use bevy::{log::info, math::Vec2};
use std::path::Path;

/// this translates too same folder as executable
pub const APP_SETTINGS_PATH: &str = "./config.toml";

fn main() {
    info!("Starting launcher: Native");
    // check AppDir for config file, if no cfg, first run, guess settings based on hardware
    // otherwise load app settings from config file
    human_panic::setup_panic!(
        human_panic::Metadata::new("AspenHalls", env!("CARGO_PKG_VERSION"))
            .authors("Hellzbellz <hellzbellz123 on github.com>")
            .homepage("https://hellzbellz123.github.io/AspenHalls")
            .support("- Open a support request on github")
    );

    let cfg_file: ConfigFile = load_settings();

    aspenlib::start_app(cfg_file).run();
}

/// loads app settings from `consts::APP_SETTINGS_PATH` and returns a boxed config file
///
/// # Panics
/// This will panic if the config file
pub fn load_settings() -> ConfigFile {
    let settings_path = Path::new(APP_SETTINGS_PATH);
    info!("loading config file from filesystem @ {:?}", settings_path);
    let target_settings = match std::fs::read_to_string(settings_path) {
        // if settings file cant be read cause it doesn't exist, no permissions, or other
        Err(error) => {
            eprintln!(
                "There was an error: {} accessing settings file at: {}",
                error,
                settings_path.display()
            );
            let new_cfg = ConfigFile::default();
            save_settings(&new_cfg, settings_path);
            toml::to_string_pretty(&new_cfg).unwrap()
        }
        // if settings file can be read
        Ok(target_settings) => target_settings,
    };

    match toml::from_str::<ConfigFile>(target_settings.as_str()) {
        // if malformed settings file, create default
        Err(error) => {
            eprintln!(
                "The app config file is malformed: {} \n this file: {}",
                error,
                settings_path.display()
            );
            ConfigFile::default()
        }
        // setting file is not malformed, can be loaded
        Ok(cfg) => {
            println!("Game Settings loaded from file successfully");
            ConfigFile {
                window_settings: WindowSettings {
                    software_cursor_enabled: cfg.window_settings.software_cursor_enabled,
                    resolution: Vec2 {
                        x: cfg.window_settings.resolution.x,
                        y: cfg.window_settings.resolution.y,
                    },
                    v_sync: cfg.window_settings.v_sync,
                    frame_rate_target: cfg.window_settings.frame_rate_target,
                    full_screen: cfg.window_settings.full_screen,
                    window_scale: cfg.window_settings.window_scale,
                    ui_scale: cfg.window_settings.ui_scale,
                },
                sound_settings: AudioSettings {
                    max_distance: 350.0,
                    max_sounds: 200,
                    volume_config: VolumeConfig {
                        master: cfg.sound_settings.volume_config.master,
                        ambience: cfg.sound_settings.volume_config.ambience,
                        music: cfg.sound_settings.volume_config.music,
                        gameplay: cfg.sound_settings.volume_config.gameplay,
                    },
                },

                general_settings: GeneralSettings {
                    enable_debug: if cfg!(feature = "develop") {
                        true
                    } else {
                        cfg.general_settings.enable_debug
                    },
                    enable_touch_controls: cfg.general_settings.enable_touch_controls,
                    camera_zoom: cfg.general_settings.camera_zoom,
                    game_difficulty: cfg.general_settings.game_difficulty,
                },
                render_settings: RenderSettings {
                    msaa: cfg.render_settings.msaa,
                },
                log_filter: cfg.log_filter,
            }
        }
    }
}
