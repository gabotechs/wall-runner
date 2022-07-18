use crate::{CameraInput, CameraState};
use bevy::prelude::*;

const STEP: f32 = 2.0;

pub fn camera_tilt(
    camera_input: Res<CameraInput>,
    time: Res<Time>,
    mut camera_state: ResMut<CameraState>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let step = STEP * time.delta().as_secs_f32();
    let diff_tilt = camera_input.tilt_angle - camera_state.tilt;
    let diff_tilt = diff_tilt.clamp(-step, step);
    camera_state.tilt += diff_tilt;
    for mut transform in query.iter_mut() {
        let yaw_quat = Quat::from_axis_angle(Vec3::Y, camera_state.yaw);
        let pitch_quat = Quat::from_axis_angle(Vec3::X, camera_state.pitch);
        let tilt_quat = Quat::from_axis_angle(Vec3::Z, camera_state.tilt);
        transform.rotation = yaw_quat * pitch_quat * tilt_quat;
    }
}

mod tests {
    use crate::startup_system_camera_setup::setup_camera;
    use crate::system_camera_tilt::camera_tilt;
    use crate::{CameraInput, CameraSettings, CameraState};
    use bevy::prelude::*;

    #[test]
    fn test_tile_camera() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins);
        app.init_resource::<CameraState>();
        app.init_resource::<CameraSettings>();
        app.insert_resource(CameraInput {
            tilt_angle: 1.0,
            ..default()
        });
        app.add_startup_system(setup_camera);
        app.add_system(camera_tilt);
        app.update();
        app.update();
        let camera_state = app.world.get_resource::<CameraState>();
        assert!(camera_state.unwrap().tilt > 0.0);
        assert!(camera_state.unwrap().tilt < 1.0);
    }
}
