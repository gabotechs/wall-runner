use crate::{CameraInput, CameraState};
use bevy::prelude::*;

const STEP: f32 = 2.0;

pub fn camera_tilt(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &CameraInput, &mut CameraState), With<Camera>>,
) {
    let step = STEP * time.delta().as_secs_f32();
    for (mut transform, camera_input, mut camera_state) in query.iter_mut() {
        let diff_tilt = camera_input.tilt_angle - camera_state.tilt;
        let diff_tilt = diff_tilt.clamp(-step, step);
        camera_state.tilt += diff_tilt;
        let yaw_quat = Quat::from_axis_angle(Vec3::Y, camera_state.yaw);
        let pitch_quat = Quat::from_axis_angle(Vec3::X, camera_state.pitch);
        let tilt_quat = Quat::from_axis_angle(Vec3::Z, camera_state.tilt);
        transform.rotation = yaw_quat * pitch_quat * tilt_quat;
    }
}

#[cfg(test)]
mod tests {
    use crate::startup_system_camera_setup::setup_camera;
    use crate::system_camera_tilt::camera_tilt;
    use crate::{CameraInput, CameraSettings, CameraState};
    use bevy::prelude::*;

    fn setup_app(app: &mut App) -> &mut App {
        app.add_plugins(MinimalPlugins)
            .init_resource::<CameraSettings>()
            .add_startup_system(setup_camera)
    }

    #[test]
    fn test_tilt_camera() {
        let mut app = App::new();
        let app = setup_app(&mut app);
        let mut camera_input_query = app.world.query::<&mut CameraInput>();
        app.add_system(camera_tilt);
        app.update();
        for mut camera_input in camera_input_query.iter_mut(&mut app.world) {
            camera_input.tilt_angle = 1.0;
        }
        app.update();
        let mut camera_state_query = app.world.query::<&mut CameraState>();

        let mut prev = 1.0;
        for camera_state in camera_state_query.iter(&app.world) {
            assert!(camera_state.tilt > 0.0);
            assert!(camera_state.tilt < 1.0);
            prev = camera_state.tilt;
        }
        app.update();
        for camera_state in camera_state_query.iter(&app.world) {
            assert!(camera_state.tilt > prev);
        }
    }
}
