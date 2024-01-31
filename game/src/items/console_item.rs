use crate::scenes::console_log;
use bevy::ecs::{component::Component, event::EventReader, system::Query};

use super::components::ItemEvent;

#[derive(Component)]
pub struct ConsoleItem(pub String);

pub fn use_item(query: Query<&ConsoleItem>, mut reader: EventReader<ItemEvent>) {
    fn item_used(ConsoleItem(string): &ConsoleItem) {
        console_log("Console Item", string);
    }

    for event in reader.read() {
        if let ItemEvent::Used { entity, .. } = event {
            if let Ok(item) = query.get(*entity) {
                item_used(item);
            }
        }
    }
}
