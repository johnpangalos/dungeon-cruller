use crate::constants::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;

use styles::elements::*;
use styles::stylesheet::*;
use styles::*;

#[derive(Component)]
pub struct RootMainMenu;

impl Plugin for RootMainMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                (ButtonStartGame::on_click, ButtonQuit::on_click)
                    .run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                despawn_recursively::<RootMainMenu>,
            );
    }
}

#[derive(Component, Default)]
struct ButtonMainMenu;
element!(ButtonMainMenu, |slot| {
    button(cn!(w_full, bg_white, hover_(bg_red_600)), [slot])
});

#[derive(Component, Default)]
struct ButtonStartGame;
element!(ButtonStartGame, ButtonMainMenu::slot);
on_click!(
    ButtonStartGame,
    (ResMut<NextState<GameState>>),
    |_, gamestate| {
        gamestate.set(GameState::Game);
    }
);

#[derive(Component, Default)]
struct ButtonQuit;
element!(ButtonQuit, ButtonMainMenu::slot);
on_click!(ButtonQuit, (EventWriter<AppExit>), |_, exit| {
    exit.send(AppExit)
});

fn setup_main_menu(mut commands: Commands) {
    let render = root(
        RootMainMenu,
        cn!(flex, justify_center, items_center, h_full, w_full),
        div(
            cn!(flex, flex_col, items_center),
            [
                ButtonStartGame::slot(text(cn!(text_5xl, text_black), "Start game")),
                ButtonQuit::slot(text(cn!(text_5xl, text_black), "Quit")),
            ],
        ),
    );

    render(&mut commands);
}

fn despawn_recursively<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
