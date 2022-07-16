use crate::system_gamepad_connections::MyGamepad;
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use wall_runner_camera::CameraControlEvent;
use wall_runner_player::PlayerControlEvent;

pub struct KeyboardSettings {
    jump: KeyCode,
    crouch: KeyCode,
    move_f: KeyCode,
    move_b: KeyCode,
    move_l: KeyCode,
    move_r: KeyCode,
    mouse_sensibility: f32,
}

impl Default for KeyboardSettings {
    fn default() -> Self {
        Self {
            jump: KeyCode::Space,
            crouch: KeyCode::LShift,
            move_f: KeyCode::W,
            move_b: KeyCode::S,
            move_l: KeyCode::A,
            move_r: KeyCode::D,
            mouse_sensibility: 8.0,
        }
    }
}

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    settings: Local<KeyboardSettings>,
    motion: Res<Events<MouseMotion>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut reader_motion: ResMut<ManualEventReader<MouseMotion>>,
    mut player_ev_writer: EventWriter<PlayerControlEvent>,
    mut camera_ev_writer: EventWriter<CameraControlEvent>,
) {
    if my_gamepad.is_some() {
        return;
    }
    let mut player_ev = PlayerControlEvent::default();
    let mut camera_ev = CameraControlEvent::default();

    if keys.pressed(settings.move_f) {
        player_ev.movement.y += 1.0;
    }
    if keys.pressed(settings.move_b) {
        player_ev.movement.y -= 1.0;
    }
    if keys.pressed(settings.move_l) {
        player_ev.movement.x += 1.0;
    }
    if keys.pressed(settings.move_r) {
        player_ev.movement.x -= 1.0;
    }
    player_ev.movement = player_ev.movement.normalize_or_zero();
    for mouse_ev in reader_motion.iter(&motion) {
        camera_ev.look.x += mouse_ev.delta.x * settings.mouse_sensibility;
        camera_ev.look.y += mouse_ev.delta.y * settings.mouse_sensibility;
    }
    if keys.pressed(settings.crouch) {
        player_ev.is_crouching = true;
    }
    if keys.just_pressed(settings.jump) {
        player_ev.jump = true;
    }
    player_ev_writer.send(player_ev);
    camera_ev_writer.send(camera_ev);
}
