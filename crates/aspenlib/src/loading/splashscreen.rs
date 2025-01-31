use bevy::prelude::*;

use crate::{loading::assets::AspenInitHandles, register_types, utilities::despawn_with, AppStage};

/// Identifies the Main Camera
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MainCamera {
    /// is this main camera used over some other camera?
    pub in_use: bool,

    /// camera movement offset scale
    pub movement_scales: Vec2,

    /// how fast should this camera move towards the player?
    pub recenter_speed: f32,

    /// multiplied by player velocity then added too the camera target
    pub look_ahead_factor: f32,

    /// change camera lerp speed when player velocity is below this magnitude
    pub lerp_change_magnitude: f32,

    /// camera speed when player is standing still
    pub player_still_recenter_speed: f32,
}

/// tag added too splashscreen entities that should be de-spawned after splashscreen
#[derive(Component)]
pub struct OnlySplashScreen;

/// `NewType` to use a `Timer` for splashscreen, if we need transitions we can use this
#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);

/// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        register_types!(app, [MainCamera]);

        app.add_systems(Startup, spawn_main_camera);
        app.add_systems(OnEnter(AppStage::Starting), splash_setup);
        app.add_systems(OnEnter(AppStage::Running), despawn_with::<OnlySplashScreen>);
    }
}

/// spawns main camera
fn spawn_main_camera(mut commands: Commands) {
    // commands.insert_resource(AmbientLight {
    //     color: Color::WHITE,
    //     brightness: 0.9,
    // });
    commands.spawn((
        Name::new("MainCamera"),
        MainCamera {
            in_use: true,
            look_ahead_factor: 0.6,
            recenter_speed: 5.0,
            lerp_change_magnitude: 0.5,
            player_still_recenter_speed: 0.05,
            movement_scales: Vec2 { x: 0.65, y: 0.65 },
        },
        // if cfg.render_settings.msaa {
        //     Msaa::Sample4
        // } else {
        //     Msaa::Off
        // },
        Camera2d,
        Camera {
            is_active: true,
            order: 1,
            hdr: false,
            clear_color: ClearColorConfig::Default,
            ..default()
        },
        // DebandDither::Enabled,
        // Tonemapping::TonyMcMapface,
        Transform::from_xyz(200.0, 100.0, 999.0),
    ));
    info!("Main Camera Spawned");
}

/// spawns splash, inserts splash timer
fn splash_setup(mut commands: Commands, init_assets: Res<AspenInitHandles>) {
    info!("spawning splash ImageBundle");
    commands.spawn((
        Name::new("SplashScreenImage"),
        OnlySplashScreen,
        Node {
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Px(0.0)),
            min_width: Val::Percent(100.0),
            min_height: Val::Percent(100.0),
            max_width: Val::Percent(100.0),
            max_height: Val::Percent(100.0),
            ..default()
        },
        ImageNode {
            image: init_assets.img_splashscreen.clone(),
            ..default()
        },
    ));
}
