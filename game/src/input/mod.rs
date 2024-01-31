use bevy::{
    ecs::system::Res,
    input::{keyboard::KeyCode, Input},
    math::Vec2,
};

pub fn input_as_axis(
    keyboard_input: Res<Input<KeyCode>>,
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,
) -> Option<Vec2> {
    let mut axis = Vec2::ZERO;

    if keyboard_input.pressed(left) {
        axis.x -= 1.0;
    }

    if keyboard_input.pressed(right) {
        axis.x += 1.0;
    }

    if keyboard_input.pressed(up) {
        axis.y += 1.0;
    }

    if keyboard_input.pressed(down) {
        axis.y -= 1.0;
    }

    if axis != Vec2::ZERO {
        return Some(axis.normalize());
    }

    return None;
}
