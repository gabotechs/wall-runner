use crate::{CameraControlEvent, CameraSettings, CameraState, GameCamera};
use bevy::prelude::*;
use wall_runner_utils::read_one_event;

pub fn camera_look(
    settings: Res<CameraSettings>,
    time: Res<Time>,
    input_ev_reader: EventReader<CameraControlEvent>,
    mut camera_state: ResMut<CameraState>,
    mut query: Query<&mut Transform, With<GameCamera>>,
) {
    let input_ev = read_one_event(input_ev_reader);
    let scale = time.delta().as_secs_f32();
    for mut transform in query.iter_mut() {
        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
        camera_state.pitch -= (settings.sensitivity * input_ev.look.y * scale).to_radians();
        camera_state.yaw -= (settings.sensitivity * input_ev.look.x * scale).to_radians();
        camera_state.pitch = camera_state.pitch.clamp(-1.54, 1.54);

        // Order is important to prevent unintended roll
        let yaw_quat = Quat::from_axis_angle(Vec3::Y, camera_state.yaw);
        let pitch_quat = Quat::from_axis_angle(Vec3::X, camera_state.pitch);
        let tilt_quat = Quat::from_axis_angle(Vec3::Z, camera_state.tilt);
        transform.rotation = yaw_quat * pitch_quat * tilt_quat;
    }
}

mod tests {
    use crate::startup_system_camera_setup::setup_camera;
    use crate::system_camera_look::camera_look;
    use crate::{CameraControlEvent, CameraSettings, CameraState, GameCamera};
    use bevy::ecs::event::Events;
    use bevy::prelude::*;
    use std::f32::consts::PI;

    fn setup_app(app: &mut App) -> &mut App {
        app.add_plugins(MinimalPlugins)
            .add_event::<CameraControlEvent>()
            .init_resource::<CameraState>()
            .init_resource::<CameraSettings>()
            .add_startup_system(setup_camera)
    }

    #[test]
    fn test_camera_look() {
        let mut app = App::new();
        let app = setup_app(&mut app);
        app.add_system(camera_look);
        app.update();
        let mut ev_writer = app.world.resource_mut::<Events<CameraControlEvent>>();
        ev_writer.send(CameraControlEvent {
            look: Vec2::new(30.0 * PI / 180.0, -30.0 * PI / 180.0),
        });
        app.update();
        let mut query = app.world.query_filtered::<&Transform, With<GameCamera>>();
        for camera in query.iter_mut(&mut app.world) {
            assert!(camera.rotation.x > 0.0);
            assert!(camera.rotation.y < 0.0);
        }
    }
}
