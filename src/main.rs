use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod world;

fn main() {
    App::new()
        .insert_resource(bevy_atmosphere::AtmosphereMat::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy_atmosphere::AtmospherePlugin {
            dynamic: false,  // Set to false since we aren't changing the sky's appearance
            sky_radius: 10.0,
        })
        .add_plugin(PlayerPlugin)
        .add_plugin(world::WorldGen)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands
) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.25,
    });

    // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}