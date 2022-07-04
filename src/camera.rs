use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Default)]
pub struct CameraState {
    pub pitch: f32,
    pub yaw: f32,
}

/// Camera position
#[derive(Default)]
pub struct CameraPosition(pub f32, pub f32, pub f32);

/// Mouse sensitivity and movement speed
pub struct CameraMovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for CameraMovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}

/// A marker component used in queries when you want flycams and not other cameras
#[derive(Component)]
pub struct FlyCam;

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(window);
    }
}

// ** cursor grab **
/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    toggle_grab_cursor(windows.get_primary_mut().unwrap());
}

/// Spawns the `Camera3dBundle` to be controlled
fn setup_camera(
    mut commands: Commands,
    initial_pos: Res<CameraPosition>,
    initial_state: Res<CameraState>,
) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(initial_pos.0, initial_pos.1, initial_pos.2)
                .with_rotation(
                    Quat::from_axis_angle(Vec3::Y, initial_state.yaw)
                        * Quat::from_axis_angle(Vec3::X, initial_state.pitch),
                ),
            ..Default::default()
        })
        .insert(FlyCam);
}

/// Handles looking around if cursor is locked
fn camera_look(
    settings: Res<CameraMovementSettings>,
    camera_pos: Res<CameraPosition>,
    windows: Res<Windows>,
    mut state: ResMut<CameraState>,
    mut reader_motion: ResMut<ManualEventReader<MouseMotion>>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    let window = windows.get_primary().unwrap();
    if !window.cursor_locked() {
        return;
    }
    let mut delta_state = state.as_mut();
    for mut transform in query.iter_mut() {
        for ev in reader_motion.iter(&motion) {
            // Using smallest of height or width ensures equal vertical and horizontal sensitivity
            let scale = window.height().min(window.width());
            delta_state.pitch -= (settings.sensitivity * ev.delta.y * scale).to_radians();
            delta_state.yaw -= (settings.sensitivity * ev.delta.x * scale).to_radians();
            delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
        }
        transform.translation.x = camera_pos.0;
        transform.translation.y = camera_pos.1;
        transform.translation.z = camera_pos.2;
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraState>()
            .init_resource::<ManualEventReader<MouseMotion>>()
            .init_resource::<CameraMovementSettings>()
            .init_resource::<CameraPosition>()
            .add_startup_system(setup_camera)
            .add_startup_system(initial_grab_cursor)
            .add_system(camera_look)
            .add_system(cursor_grab);
    }
}

// #[derive(Component)]
// pub struct CameraFollow;
//
// pub fn camera_follow(mut query: Query<&mut Transform, With<CameraFollow>>) {
//     for mut transform in query.iter_mut() {}
// }
