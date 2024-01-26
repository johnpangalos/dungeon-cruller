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
        app.add_systems(OnEnter(GameState::Paused), setup_main_menu)
            .add_systems(
                Update,
                (ButtonBack::on_click(), ButtonQuit::on_click())
                    .run_if(in_state(GameState::Paused)),
            )
            .add_systems(
                OnExit(GameState::Paused),
                despawn_recursively::<PausePlugin>,
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
struct ButtonBack;
render!(ButtonBack, |_, slot| button(
    cn!(w_full, bg_white, hover_(bg_red_600), pressed_(bg_red_800)),
    slot
));
on_click!(
    ButtonBack,
    (ResMut<NextState<GameState>>),
    |_, gamestate| {
        gamestate.set(GameState::Running);
    }
);

#[derive(Component, Default)]
struct ButtonQuit;
render!(ButtonQuit, |_, slot| button(
    cn!(w_full, bg_white, hover_(bg_red_600), pressed_(bg_red_800)),
    slot
));
on_click!(ButtonQuit, (EventWriter<AppExit>), |_, exit| {
    exit.send(AppExit)
});

fn setup_main_menu(mut commands: Commands) {
    let tree = div(
        cn!(h_full, w_full, flex, justify_center, items_center),
        div(
            cn!(flex, flex_col),
            [
                ButtonBack.slot(text(cn!(text_5xl, text_black), "Go back to game")),
                ButtonQuit.slot(text(cn!(text_5xl, text_black), "Quit")),
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
