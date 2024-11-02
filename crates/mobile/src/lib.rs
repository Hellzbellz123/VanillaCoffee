#![doc = r"
    mobile library too be used by platform specific apps
    currently only targets android, should be expanded too ios mobile
"]

use aspenlib::{
    ConfigFile, GameDifficulty, GeneralSettings, RenderSettings, SoundSettings, WindowSettings,
};
use bevy::{math::Vec2, prelude::bevy_main};

#[bevy_main]
fn main() {
    let config = ConfigFile {
        log_filter: Some("Info,wgpu=error,naga=error".to_string()),
        window_settings: WindowSettings {
            software_cursor_enabled: true,
            v_sync: true,
            frame_rate_target: 120.0,
            full_screen: true,
            resolution: Vec2 {
                x: 1920.0,
                y: 1080.0,
            },
            window_scale: 2.0,
            ui_scale: 1.0,
        },
        sound_settings: SoundSettings {
            master_volume: 1.0,
            ambience_volume: 0.5,
            music_volume: 0.5,
            sound_volume: 0.5,
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
