use crate::input::event_input::InputEvent;
use crate::input::system_gamepad_connections::MyGamepad;
use bevy::prelude::*;

pub struct GamepadSettings {
    jump: GamepadButtonType,
    crouch: GamepadButtonType,
    move_x: GamepadAxisType,
    move_y: GamepadAxisType,
    look_x: GamepadAxisType,
    look_y: GamepadAxisType,
    look_sensibility_x: f32,
    look_sensibility_y: f32,
}

impl Default for GamepadSettings {
    fn default() -> Self {
        Self {
            jump: GamepadButtonType::LeftTrigger,
            crouch: GamepadButtonType::LeftTrigger2,
            move_x: GamepadAxisType::LeftStickX,
            move_y: GamepadAxisType::LeftStickY,
            look_x: GamepadAxisType::RightStickX,
            look_y: GamepadAxisType::RightStickY,
            look_sensibility_x: 100.0,
            look_sensibility_y: 40.0,
        }
    }
}

pub fn gamepad_input(
    settings: Local<GamepadSettings>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut ev_writer: EventWriter<InputEvent>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return;
    };

    let mut ev = InputEvent::default();

    let axis_lx = GamepadAxis(gamepad, settings.move_x);
    let axis_ly = GamepadAxis(gamepad, settings.move_y);
    let axis_rx = GamepadAxis(gamepad, settings.look_x);
    let axis_ry = GamepadAxis(gamepad, settings.look_y);

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        ev.movement = Vec2::new(-x, y);
    }
    if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
        ev.look = Vec2::new(
            x * settings.look_sensibility_x,
            -y * settings.look_sensibility_y,
        );
    }

    // In a real game, the buttons would be configurable, but here we hardcode them
    let jump_button = GamepadButton(gamepad, settings.jump);
    let crouch_button = GamepadButton(gamepad, settings.crouch);

    if buttons.just_pressed(jump_button) {
        ev.jump = true;
    }

    if buttons.pressed(crouch_button) {
        ev.is_crouching = true;
    }

    ev_writer.send(ev);
}
