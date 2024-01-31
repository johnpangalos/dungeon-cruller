mod casual_bullet_item;
pub mod components;
mod console_item;

use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};
pub use console_item::ConsoleItem;

use crate::constants::AppSet;

use self::components::ItemEvent;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemEvent>().add_systems(
            Update,
            (console_item::use_item, casual_bullet_item::use_item)
                .in_set(AppSet::Items)
                .after(AppSet::Player),
        );
    }
}
