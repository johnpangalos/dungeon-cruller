use std::hash::Hash;

use bevy::input::common_conditions::input_just_pressed;
use bevy::input::common_conditions::input_pressed;
use bevy::prelude::*;
use styles::elements::*;
use styles::stylesheet::*;
use styles::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum DebugState {
    #[default]
    Hidden,
    Visible,
}

/**
 * This is the debug overlay. You can print to it using the `console_log` function.
 * F1 shows the overlay, F2 clears it.
 */

const ARRAY_REPEAT_VALUE: Option<(String, String)> = None;
static mut DEBUG: [Option<(String, String)>; 100] = [ARRAY_REPEAT_VALUE; 100];
static mut DEBUG_CHANGED: bool = true;

/**
 * Prints a key-value pair to the debug overlay.
 */
#[allow(dead_code)]
pub fn console_log(key: impl ToString, value: impl ToString) {
    let key_string = key.to_string();
    for line in unsafe { DEBUG.iter_mut() } {
        match line {
            None => {
                *line = Some((key_string, value.to_string()));
                unsafe {
                    DEBUG_CHANGED = true;
                }

                return;
            }
            Some((k, existing_value)) if *k == key_string => {
                if *existing_value == value.to_string() {
                    return;
                }

                *line = Some((key_string, value.to_string()));
                unsafe {
                    DEBUG_CHANGED = true;
                }

                return;
            }
            _ => {}
        }
    }
}

#[derive(Component)]

pub struct DebugOverlay;

impl Plugin for DebugOverlay {
    fn build(&self, app: &mut App) {
        app.add_state::<DebugState>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(DebugState::Visible), show_component::<DebugOverlay>)
            .add_systems(OnEnter(DebugState::Hidden), hide_component::<DebugOverlay>)
            .add_systems(
                Update,
                (
                    show_console.run_if(in_state(DebugState::Hidden)),
                    hide_console.run_if(in_state(DebugState::Visible)),
                )
                    .run_if(input_just_pressed(KeyCode::F1)),
            )
            .add_systems(
                Update,
                (
                    zoom_out_console_log.run_if(
                        input_pressed(KeyCode::ControlLeft)
                            .and_then(input_just_pressed(KeyCode::NumpadSubtract)),
                    ),
                    zoom_in_console_log.run_if(
                        input_pressed(KeyCode::ControlLeft)
                            .and_then(input_just_pressed(KeyCode::NumpadAdd)),
                    ),
                    write_console_log,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                clear_console.run_if(input_just_pressed(KeyCode::F2)),
            );
    }
}

pub fn clear_console() {
    unsafe {
        DEBUG = [ARRAY_REPEAT_VALUE; 100];
    }
}

pub fn show_console(mut debug_state: ResMut<NextState<DebugState>>) {
    debug_state.set(DebugState::Visible);
}

pub fn hide_console(mut debug_state: ResMut<NextState<DebugState>>) {
    debug_state.set(DebugState::Hidden);
}

#[derive(Component, Clone)]
struct List;

fn setup(mut commands: Commands) {
    let tree = div(
        cn!(flex, flex_col, h_full, w_full, overflow_hidden),
        List.as_el(text(cn!(text_2xl), "")),
    );

    spawn_root_element(&mut commands, DebugOverlay, tree);
}

fn write_console_log(mut query: Query<&mut Text, With<List>>) {
    unsafe {
        let debug_changed = DEBUG_CHANGED.clone();
        DEBUG_CHANGED = false;
        if !debug_changed {
            return;
        }

        for mut text in &mut query {
            let style = text.sections[0].style.clone();
            let mut sections = vec![TextSection::new("DEBUG\n", style.clone())];

            for val in DEBUG.iter() {
                if let Some((key, value)) = val {
                    sections.push(TextSection::new(
                        format!("{}: {}\n", key, value),
                        style.clone(),
                    ));
                }
            }

            text.sections = sections;
        }
    }
}

fn zoom_out_console_log(mut query: Query<&mut Text, With<List>>) {
    for mut text in &mut query {
        for section in &mut text.sections {
            section.style.font_size -= 1.;
        }
    }
}

fn zoom_in_console_log(mut query: Query<&mut Text, With<List>>) {
    for mut text in &mut query {
        for section in &mut text.sections {
            section.style.font_size += 1.;
        }
    }
}

fn hide_component<T: Component>(mut to_hide: Query<&mut Visibility, With<T>>) {
    for mut visibility in &mut to_hide {
        *visibility = Visibility::Hidden;
    }
}

fn show_component<T: Component>(mut to_show: Query<&mut Visibility, With<T>>) {
    for mut visibility in &mut to_show {
        *visibility = Visibility::Visible;
    }
}
