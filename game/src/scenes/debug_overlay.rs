use crate::constants::DebugState;
use bevy::ecs::system::Insert;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use styles::elements::*;
use styles::stylesheet::*;
use styles::*;

const ARRAY_REPEAT_VALUE: Option<(&'static str, String)> = None;
static mut DEBUG: [Option<(&'static str, String)>; 100] = [ARRAY_REPEAT_VALUE; 100];

pub fn console_log(key: &'static str, value: impl ToString + 'static) {
    for line in unsafe { DEBUG.iter_mut() } {
        match line {
            None => {
                *line = Some((key, value.to_string()));
                return;
            }
            Some((k, _)) if *k == key => {
                *line = Some((key, value.to_string()));
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
            .add_systems(OnEnter(DebugState::Visible), show::<DebugPlugin>)
            .add_systems(Update, write_debug_system)
            .add_systems(
                Update,
                (
                    show_debug.run_if(in_state(DebugState::Visible)),
                    hide_debug.run_if(in_state(DebugState::Hidden)),
                )
                    .run_if(input_just_pressed(KeyCode::F1)),
            )
            .add_systems(OnExit(DebugState::Hidden), hide::<DebugPlugin>);
    }
}

pub fn show_debug(mut debug_state: ResMut<NextState<DebugState>>) {
    debug_state.set(DebugState::Visible);
}

pub fn hide_debug(mut debug_state: ResMut<NextState<DebugState>>) {
    debug_state.set(DebugState::Hidden);
}

#[derive(Component, Clone)]
struct List;
render!(List, |_, _| text(cn!(text_4xl), ""));

fn setup(mut commands: Commands) {
    let tree = div(cn!(flex, flex_col), List.el());

    render_root(&mut commands, DebugPlugin, tree);
}

fn write_debug_system(mut query: Query<&mut Text, With<List>>) {
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

fn hide<T: Component>(mut to_hide: Query<&mut Visibility, With<T>>) {
    for mut visibility in &mut to_hide {
        *visibility = Visibility::Hidden;
    }
}

fn show<T: Component>(mut to_show: Query<&mut Visibility, With<T>>) {
    for mut visibility in &mut to_show {
        *visibility = Visibility::Visible;
    }
}
