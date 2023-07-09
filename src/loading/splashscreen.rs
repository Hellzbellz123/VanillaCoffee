use bevy::{
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    prelude::*,
};
// use rust_embed::RustEmbed;
use crate::{game::GameStage, loading::assets::SPLASHASSETPATH};

/// Identifies the Main Camera
#[derive(Component, Reflect)]
pub struct MainCameraTag {
    /// true if active, false if not
    pub is_active: bool,
}

/// tag added too splashscreen entitys that should be despawned after splashscreen
#[derive(Component)]
pub struct OnlySplashScreen;

/// Newtype to use a `Timer` for splashscreen
#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);

/// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // TODO: do some speciial trickery to make this system work awesome
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`

        app.add_systems((
            spawn_main_camera.in_schedule(OnEnter(GameStage::Loading)),
            splash_setup.in_schedule(OnEnter(GameStage::Loading)),
        ));
    }
}

/// spawns maincamera
fn spawn_main_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                is_active: true,
                order: 1,
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::AcesFitted,
            deband_dither: DebandDither::Enabled,
            ..default()
        },
        Name::new("MainCamera"),
        MainCameraTag { is_active: true },
    ));
    info!("Main Camera Spawned");
}

/// spawns splash, inserts splash timer
fn splash_setup(mut commands: Commands, assetserver: ResMut<AssetServer>) {
    info!("loading splash");
    let img = assetserver.load(SPLASHASSETPATH);

    // Display the logo
    info!("spawning splash ImageBundle");
    commands
        .spawn(ImageBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            image: UiImage {
                texture: img,
                ..default()
            },
            ..default()
        })
        .insert(OnlySplashScreen);
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}
