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

fn initial_setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        brightness: 0.9,
        ..default()
    });
    // commands.spawn_bundle(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(8.0, 8.0, 8.0),
    //     ..default()
    // });
}
