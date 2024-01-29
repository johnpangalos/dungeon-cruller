use crate::constants;
use bevy::prelude::*;

pub enum DoorLocation {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Bundle)]
pub struct DoorBundle {
    sprite_bundle: SpriteBundle,
}

#[derive(Event, Default)]
pub struct CollisionEvent;

impl DoorLocation {
    fn position(&self) -> Vec2 {
        match self {
            DoorLocation::Left => Vec2::new(-100., 0.),
            DoorLocation::Right => Vec2::new(100., 0.),
            DoorLocation::Bottom => Vec2::new(0., 100.),
            DoorLocation::Top => Vec2::new(0., -100.),
        }
    }
    fn size(&self) -> Vec2 {
        match self {
            DoorLocation::Right => Vec2::new(100., 100.),
            DoorLocation::Left => Vec2::new(100., 100.),
            DoorLocation::Top => Vec2::new(100., 100.),
            DoorLocation::Bottom => Vec2::new(100., 100.),
        }
    }
}

impl DoorBundle {
    pub fn new(location: DoorLocation) -> DoorBundle {
        let size = location.size();
        DoorBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(location.position().extend(2.)),
                sprite: Sprite {
                    color: constants::DOOR_COLOR,
                    custom_size: Some(size.clone()),
                    ..default()
                },
                ..default()
            },
        }
    }
}
