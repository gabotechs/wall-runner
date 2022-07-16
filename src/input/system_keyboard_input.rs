use crate::input::event_input::InputEvent;
use crate::input::system_gamepad_connections::MyGamepad;
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

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
    mut ev_writer: EventWriter<InputEvent>,
) {
    if my_gamepad.is_some() {
        return;
    }
    let mut ev = InputEvent::default();

    if keys.pressed(settings.move_f) {
        ev.movement.y += 1.0;
    }
    if keys.pressed(settings.move_b) {
        ev.movement.y -= 1.0;
    }
    if keys.pressed(settings.move_l) {
        ev.movement.x += 1.0;
    }
    if keys.pressed(settings.move_r) {
        ev.movement.x -= 1.0;
    }
    ev.movement = ev.movement.normalize_or_zero();
    for mouse_ev in reader_motion.iter(&motion) {
        ev.look.x += mouse_ev.delta.x * settings.mouse_sensibility;
        ev.look.y += mouse_ev.delta.y * settings.mouse_sensibility;
    }
    if keys.pressed(settings.crouch) {
        ev.is_crouching = true;
    }
    if keys.just_pressed(settings.jump) {
        ev.jump = true;
    }
    ev_writer.send(ev);
}
