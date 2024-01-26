use crate::constants::DebugState;
use bevy::ecs::system::Insert;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use styles::elements::*;
use styles::stylesheet::*;
use styles::*;

/**
 * This is the debug overlay. You can print to it using the `console_log` function.
 * F1 shows the overlay, F2 clears it.
 */

const ARRAY_REPEAT_VALUE: Option<(String, String)> = None;
static mut DEBUG: [Option<(String, String)>; 100] = [ARRAY_REPEAT_VALUE; 100];

pub fn console_log(key: impl ToString, value: impl ToString) {
    let key_string = key.to_string();
    for line in unsafe { DEBUG.iter_mut() } {
        match line {
            None => {
                *line = Some((key_string, value.to_string()));
                return;
            }
            Some((k, _)) if *k == key_string => {
                *line = Some((key_string, value.to_string()));
                return;
            }
            _ => {}
        }
    }
}

#[derive(Component)]

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(DebugState::Visible), show_component::<DebugPlugin>)
            .add_systems(
                OnEnter(DebugState::Hidden),
                (hide_component::<DebugPlugin>, hide_aabbs),
            )
            .add_systems(Update, write_console_log)
            .add_systems(Update, show_aabbs.run_if(in_state(DebugState::Visible)))
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
render!(List, |_, _| text(cn!(text_4xl), ""));

fn setup(mut commands: Commands) {
    let tree = div(cn!(flex, flex_col), List.el());

    render_root(&mut commands, DebugPlugin, tree);
}

fn write_console_log(mut query: Query<&mut Text, With<List>>) {
    unsafe {
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

fn show_aabbs(mut commands: Commands, aabbs: Query<Entity, (With<Aabb>, Without<AabbGizmo>)>) {
    for entity in &aabbs {
        commands.entity(entity).insert(AabbGizmo::default());
    }
}

fn hide_aabbs(mut commands: Commands, aabbs: Query<Entity, (With<Aabb>, With<AabbGizmo>)>) {
    for entity in &aabbs {
        commands.entity(entity).remove::<AabbGizmo>();
    }
}
