use crate::constants::GameState;
use bevy::app::AppExit;
use bevy::ecs::system::Insert;
use bevy::prelude::*;

use styles::elements::*;
use styles::stylesheet::*;
use styles::*;

#[derive(Component)]
pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), setup)
            .add_systems(
                Update,
                (Back::on_click(), Quit::on_click()).run_if(in_state(GameState::Paused)),
            )
            .add_systems(
                OnExit(GameState::Paused),
                despawn_recursively::<PausePlugin>,
            );
    }
}

#[derive(Component, Clone)]
struct Back {
    label: &'static str,
}
render!(Back, |&Back { label }, slot| button(
    cn!(w_full, bg_white, hover_(bg_red_600), pressed_(bg_red_800)),
    [text(cn!(text_5xl, text_black), label), slot]
));
on_click!(Back, (ResMut<NextState<GameState>>), |_, gamestate| {
    gamestate.set(GameState::Running);
});

#[derive(Component, Clone)]
struct Quit {
    label: &'static str,
}
render!(Quit, |&Quit { label }, slot| button(
    cn!(w_full, bg_white, hover_(bg_red_600), pressed_(bg_red_800)),
    [text(cn!(text_5xl, text_black), label), slot]
));
on_click!(Quit, (EventWriter<AppExit>), |_, exit| {
    exit.send(AppExit)
});

fn setup(mut commands: Commands) {
    let tree = div(
        cn!(h_full, w_full, flex, justify_center, items_center),
        div(
            cn!(flex, flex_col),
            [
                Back {
                    label: "Go back to game",
                }
                .el(),
                Quit { label: "Quit" }.el(),
            ],
        ),
    );

    render_root(&mut commands, PausePlugin, tree);
}

fn despawn_recursively<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
