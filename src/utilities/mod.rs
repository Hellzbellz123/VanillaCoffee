use crate::{
    audio::SoundSettings, components::MainCameraTag, utilities::game::AppSettings,
    APP_SETTINGS_PATH,
};
use bevy::{prelude::*, window::PrimaryWindow};
use std::{ops::Mul, path::Path, thread};

pub mod game;
pub mod logging;
pub mod window;

/// holds general game utilities
/// not particularly related to gameplay
pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dev")]
        app.add_system(window::set_debug_title);

        app.add_startup_system(window::set_window_icon)
            .insert_resource(EagerMousePos {
                world: Vec2::ZERO,
                window: Vec2::ZERO,
            })
            .add_system(eager_cursor_pos);
    }
}

/// mouse position eagerly updated for latency focused stuff. probably bad, will find out
#[derive(Debug, Resource, Clone, Copy, PartialEq, Component)]
pub struct EagerMousePos {
    /// mouse pos in world space
    pub world: Vec2,
    /// mouse pos in window coords
    pub window: Vec2,
}
/// updates `EagerMousePos` resource with current mouse positon and mouse pos translated to worldspace. no change detection, always runs
fn eager_cursor_pos(
    mut fastmousepos: ResMut<EagerMousePos>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCameraTag>>,
    main_window: Query<&Window, With<PrimaryWindow>>,
) {
    if q_camera.is_empty() {
        return;
    }
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // // Games typically only have one window (the primary window).
    // // For multi-window applications, you need to use a specific window ID here.
    let wnd = main_window.single();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width(), wnd.height());

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        fastmousepos.world = world_pos;
        fastmousepos.window = screen_pos;
        // info!("eager mouse update {}", world_pos);
    } else {
        fastmousepos.world = Vec2::ZERO;
        fastmousepos.window = Vec2::ZERO;
        // info!("eager mouse not in window");
    }
}

pub fn despawn_with<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    to_despawn.for_each(|entity| {
        info!("despawning entity recursively: {:#?}", entity);
        commands.entity(entity).despawn_recursive();
    });
}

#[must_use]
pub fn load_settings() -> AppSettings {
    let settings_path = Path::new(APP_SETTINGS_PATH);
    let settings_file_path = std::fs::read_to_string(settings_path); //File::open(settings_path);

    match settings_file_path {
        // if settings file cant be read cause it doesnt exit, no permissions, or other
        Err(target_settings) => {
            info!(
                "there was an error: {} acessing settings file as: {}",
                target_settings,
                settings_path.display()
            );
            let settings = create_default_settings();
            settings
        }
        // if settings file can be read
        Ok(target_settings) => {
            let toml_cfg: AppSettings =
                match toml::from_str::<AppSettings>(target_settings.as_str()) {
                    // if malformed settings file, create default
                    Err(toml_cfg) => {
                        info!(
                            "There was an error deserializing `AppSettings`: {} at {}",
                            toml_cfg,
                            settings_path.display()
                        );
                        let settings = create_default_settings();
                        settings
                    }
                    // setting file is not malformed, can be loaded
                    Ok(toml_cfg) => {
                        info!("Game Settings loaded from file succesfully");
                        AppSettings {
                            vsync: toml_cfg.vsync,
                            frame_rate_target: toml_cfg.frame_rate_target,
                            camera_zoom: toml_cfg.camera_zoom,
                            resolution: toml_cfg.resolution,
                            sound_settings: toml_cfg.sound_settings,
                            fullscreen: toml_cfg.fullscreen,
                        }
                    }
                };
            toml_cfg
        }
    }
}

fn create_default_settings() -> AppSettings {
    let settings_path = Path::new(APP_SETTINGS_PATH);
    let app_settings = AppSettings {
        camera_zoom: 1.0,
        resolution: Vec2 {
            x: 1200.0,
            y: 900.0,
        },
        sound_settings: SoundSettings {
            mastervolume: 1.0,
            ambiencevolume: 1.0,
            musicvolume: 1.0,
            soundvolume: 1.0,
        },
        vsync: true,
        frame_rate_target: 60.0,
        fullscreen: false,
    };

    let thread_one = thread::spawn(move || save_settings(app_settings, settings_path));

    if thread_one.is_finished() {
        thread_one.join().expect("coulddnt join thread");
        info!("MultiThreaded save complete");
    }
    app_settings
}

fn save_settings(app_settings: AppSettings, settings_path: &Path) {
    info!("Saving AppSettings, this overwrites current settings");
    let serd_cfg = toml::to_string(&app_settings).expect("error converting config to string");
    std::fs::write(settings_path, serd_cfg).expect("couldnt write file");
}

/// Performs a linear interpolation between `from` and `to` based on the value `s`.
///
/// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
/// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
/// extrapolated.
#[must_use]
pub fn lerp<T>(from: T, to: T, s: T) -> T
where
    <T as std::ops::Sub>::Output: Mul<T>,
    T: std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::marker::Copy,
{
    from + ((to - from) * s)
}
