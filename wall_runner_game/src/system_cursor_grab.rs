use bevy::prelude::*;
use wall_runner_camera::CameraInput;

pub fn cursor_grab(
    mut initial_grab: Local<bool>,
    keys: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut camera_input: ResMut<CameraInput>,
) {
    let window = windows.get_primary_mut().unwrap();
    if !*initial_grab {
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
        *initial_grab = true;
    }
    if keys.just_pressed(KeyCode::Escape) {
        camera_input.inactive = !camera_input.inactive;
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}
