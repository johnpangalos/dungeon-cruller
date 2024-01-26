use crate::constants::GameState;
use bevy::app::AppExit;
use bevy::ecs::system::Insert;
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

macro_rules! render {
    ($component:ident, $element:expr) => {
        impl Render for $component {
            fn render(&self, parent: &mut ChildBuilder, slot: Element) -> Entity {
                let e = render(parent, $element(&self, slot));

                parent.add_command(Insert {
                    entity: e,
                    bundle: $component,
                });

                e
            }
        }
    };
}

#[derive(Component, Default)]
struct ButtonMainMenu;
render!(ButtonMainMenu, |_, slot| button(
    cn!(w_full, bg_white, hover_(bg_red_600)),
    slot
));

#[derive(Component, Default)]
struct ButtonStartGame;
render!(ButtonStartGame, |_, slot| ButtonMainMenu.slot(slot));
on_click!(
    ButtonStartGame,
    (ResMut<NextState<GameState>>),
    |_, gamestate| {
        gamestate.set(GameState::Game);
    }
);

#[derive(Component, Default)]
struct ButtonQuit;
render!(ButtonQuit, |_, slot| ButtonMainMenu.slot(slot));
on_click!(ButtonQuit, (EventWriter<AppExit>), |_, exit| {
    exit.send(AppExit)
});

fn setup_main_menu(mut commands: Commands) {
    let tree = div(
        cn!(h_full, w_full, flex, justify_center, items_center),
        div(
            cn!(flex, flex_col),
            [
                ButtonStartGame.slot(text(cn!(text_5xl, text_black), "Start game")),
                ButtonQuit.slot(text(cn!(text_5xl, text_black), "Quit")),
            ],
        ),
    );

    render_root(&mut commands, RootMainMenu, tree);
}

fn despawn_recursively<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
