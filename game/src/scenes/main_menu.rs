use crate::constants::AppState;
use bevy::app::AppExit;
use bevy::ecs::system::Insert;
use bevy::prelude::*;

use styles::elements::*;
use styles::stylesheet::*;
use styles::*;

#[derive(Component)]

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), setup)
            .add_systems(
                Update,
                (Start::on_click(), Quit::on_click()).run_if(in_state(AppState::Splash)),
            )
            .add_systems(
                OnExit(AppState::Splash),
                despawn_recursively::<SplashPlugin>,
            );
    }
}

#[derive(Component, Clone)]
struct Start(&'static str);

render!(Start, |&Start(label), slot| button(
    cn!(w_full, bg_white, hover_(bg_red_600), pressed_(bg_red_800)),
    [text(cn!(text_5xl, text_black), label), slot]
));
on_click!(Start, (ResMut<NextState<AppState>>), |_, gamestate| {
    gamestate.set(AppState::Game);
});

#[derive(Component, Clone)]
struct Quit(&'static str);

render!(Quit, |&Quit(label), slot| button(
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
            [Start("Start game").el(), Quit("Quit").el()],
        ),
    );

    render_root(&mut commands, SplashPlugin, tree);
}

fn despawn_recursively<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
