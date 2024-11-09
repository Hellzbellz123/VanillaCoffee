#![doc = r"
    mobile library too be used by platform specific apps
    currently only targets android, should be expanded too ios mobile
"]

use aspenlib::{
    AudioSettings, ConfigFile, GameDifficulty, GeneralSettings, RenderSettings, VolumeConfig,
    WindowSettings,
};
use bevy::{math::Vec2, prelude::bevy_main};

// TODO: switch to android GameActivity when bevy 0.15 releases
#[bevy_main]
fn main() {
    let config = ConfigFile {
        log_filter: Some("Info,wgpu=error,naga=error".to_string()),
        window_settings: WindowSettings {
            software_cursor_enabled: true,
            v_sync: true,
            frame_rate_target: Some(120.0),
            full_screen: true,
            resolution: Vec2 {
                x: 1920.0,
                y: 1080.0,
            },
            window_scale: 2.0,
            ui_scale: 1.0,
        },
        sound_settings: AudioSettings {
            max_distance: 350.0,
            max_sounds: 200,
            volume_config: VolumeConfig {
                master: 0.5,
                gameplay: 1.0,
                ambience: 1.0,
                music: 1.0,
            },
        },
        general_settings: GeneralSettings {
            enable_debug: if cfg!(feature = "develop") {
                true
            } else {
                false
            },
            enable_touch_controls: true,
            camera_zoom: 5.5,
            game_difficulty: GameDifficulty::Medium,
        },
        render_settings: RenderSettings { msaa: false },
    };

    println!("Starting launcher: Mobile");
    aspenlib::start_app(config).run();
}
