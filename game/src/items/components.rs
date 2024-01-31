use bevy::{
    ecs::{component::Component, entity::Entity, event::Event},
    math::Vec2,
};

#[derive(Event)]
pub enum ItemEvent {
    Used {
        entity: Entity,
        position: Vec2,
        direction: Vec2,
    },
    Dropped {
        entity: Entity,
        position: Vec2,
    },
}

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Cooldown(f32);

#[derive(Component)]
pub struct Name(String);

#[derive(Component, Default)]
pub struct Bullet;

#[derive(Component, Default)]
pub struct Speed(pub f32);

#[derive(Component, Default)]
pub struct Lifetime(pub f32);
