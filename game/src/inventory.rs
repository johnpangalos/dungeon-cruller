use std::collections::VecDeque;

use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        system::{Commands, Query},
    },
    hierarchy::Children,
};

use crate::scenes::console_log;

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Cooldown(f32);

#[derive(Component)]
pub struct ConsoleItem(pub String);

pub fn use_console_item(query: Query<&ConsoleItem>, mut reader: EventReader<ItemEvent>) {
    for event in reader.read() {
        match event {
            ItemEvent::Used(entity) => {
                if let Ok(ConsoleItem(string)) = query.get(*entity) {
                    console_log("Console Item", string);
                }
            }
            _ => {}
        }
    }
}

#[derive(Component)]
pub enum Inventory {
    OneHanded(Option<Entity>),
    DoubleHanded(Option<Entity>, Entity),
    Revolver(VecDeque<Entity>),
}

#[derive(Event)]
pub enum ItemEvent {
    Used(Entity),
    Dropped(Entity),
}

pub fn use_active_item(inventory: &mut Inventory, writer: &mut EventWriter<ItemEvent>) {
    match inventory {
        Inventory::DoubleHanded(Some(entity), _) => {
            writer.send(ItemEvent::Used(entity.clone()));
        }
        Inventory::Revolver(entities) => {
            if let Some(entity) = entities.front() {
                writer.send(ItemEvent::Used(entity.clone()));
            }

            entities.rotate_left(1);
        }
        Inventory::OneHanded(Some(entity)) => {
            writer.send(ItemEvent::Used(entity.clone()));
        }
        _ => {}
    };
}
