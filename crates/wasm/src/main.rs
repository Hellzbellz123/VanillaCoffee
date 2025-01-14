#![doc = r"
    web app built with yew too hold the bevy application
"]

use aspenlib::*;
use bevy::{log::info, math::Vec2};
use log::Level;
use yew::prelude::*;

/// sets browser window title too passed string
fn set_window_title(title: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            document.set_title(title);
        }
    }
}

#[function_component(Root)]
fn view() -> Html {
    set_window_title("Aspen Halls");

    html! {
        <> </>
    }
}

fn main() {
    #[cfg(feature = "develop")]
    wasm_logger::init(wasm_logger::Config::new(Level::Info).module_prefix("aspenlib"));

    // Mount the DOM
    yew::Renderer::<Root>::new().render();

    // Start the Bevy App
    info!("Starting launcher: WASM");
    let cfg_file = ConfigFile {
        log_filter: Some("Info,wgpu=error,naga=error".to_string()),
        window_settings: WindowSettings {
            software_cursor_enabled: true,
            v_sync: true,
            frame_rate_target: Some(60.0),
            full_screen: false,
            resolution: Vec2::new(1280.0, 720.0),
            window_scale: 1.0,
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
            enable_debug: cfg!(feature = "develop"),
            enable_touch_controls: false,
            camera_zoom: 3.5,
            game_difficulty: GameDifficulty::Easy,
        },
        render_settings: RenderSettings { msaa: false },
    };

    aspenlib::start_app(cfg_file).run();
}
