use bevy::{
    ecs::{component::Component, entity::Entity, event::Event},
    math::{cubic_splines::CubicCurve, Quat, Vec2},
};

#[derive(Event)]
pub enum ItemEvent {
    Used {
        entity: Entity,
        position: Vec2,
        rotation: Quat,
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
pub struct Trajectory(pub Vec<TrajectorySegment>);

#[derive(Debug)]
pub struct TrajectorySegment(pub Vec2, pub f32);

impl Trajectory {
    pub fn straight(start: Vec2, direction: Vec2, length: f32) -> Self {
        Self(vec![
            TrajectorySegment(start, 0.),
            TrajectorySegment(start + direction * length, 1.),
        ])
    }
}

#[derive(Component, Default)]
pub struct Lifetime {
    pub current: f32,
    pub lifespan: f32,
}
