use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod world;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
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
    /*
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    });
    */

    commands.insert_resource(DirectionalLight {
        shadows_enabled: true,
        ..Default::default()
    });
}