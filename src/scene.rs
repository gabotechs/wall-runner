use bevy::prelude::*;
use bevy_atmosphere::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AtmosphereMat::default())
            .add_plugin(AtmospherePlugin::default())
            .add_startup_system(initial_setup);
    }
}

fn initial_setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        brightness: 1.0,
        ..default()
    });
}
