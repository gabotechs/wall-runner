use bevy::prelude::*;
use wall_runner_events::*;

pub fn cursor_grab(
    keys: Res<Input<KeyCode>>,
    mut initial_grab: Local<bool>,
    mut windows: ResMut<Windows>,
    mut pause_ev_writer: EventWriter<PauseEvent>,
    mut resume_ev_writer: EventWriter<ResumeEvent>,
) {
    let window = windows.get_primary_mut().unwrap();
    if !*initial_grab {
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
        *initial_grab = true;
    }
    if keys.just_pressed(KeyCode::Escape) {
        let was_running = window.cursor_locked();
        if was_running {
            pause_ev_writer.send(PauseEvent);
        } else {
            resume_ev_writer.send(ResumeEvent);
        }
        window.set_cursor_lock_mode(!was_running);
        window.set_cursor_visibility(!window.cursor_visible());
    }
}
