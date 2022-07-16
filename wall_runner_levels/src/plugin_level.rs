use crate::resource_level_status::LevelStatus;
use crate::system_choose_level::choose_level;
use crate::system_level_status::level_status;
use crate::{EventLevelFinish, LevelPlayerPositionInput};
use bevy::prelude::*;
use bevy_atmosphere::{AtmosphereMat, AtmospherePlugin};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AtmosphereMat {
            planet_radius: AtmosphereMat::default().planet_radius - 5000.0,
            ..default()
        })
        .add_event::<EventLevelFinish>()
        .init_resource::<LevelPlayerPositionInput>()
        .init_resource::<LevelStatus>()
        .add_plugin(AtmospherePlugin::default())
        .add_startup_system(spawn_environment)
        .add_system(choose_level)
        .add_system(level_status);
    }
}

fn spawn_environment(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        brightness: 1.0,
        ..default()
    });
}
