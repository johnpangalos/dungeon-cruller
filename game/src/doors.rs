use crate::constants::{
    self, BOTTOM_WALL, DOOR_WIDTH, LEFT_WALL, RIGHT_WALL, TOP_WALL, WALL_THICKNESS,
};
use bevy::{prelude::*, sprite::Anchor};

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
        let wall_position = match self {
            DoorLocation::Left => Vec2::new(LEFT_WALL, 0.),
            DoorLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            DoorLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            DoorLocation::Top => Vec2::new(0., TOP_WALL),
        };
        wall_position
    }

    fn anchor(&self) -> Anchor {
        match self {
            DoorLocation::Left => Anchor::CenterLeft,
            DoorLocation::Right => Anchor::CenterRight,
            DoorLocation::Bottom => Anchor::BottomCenter,
            DoorLocation::Top => Anchor::TopCenter,
        }
    }
}

impl DoorBundle {
    pub fn new(location: DoorLocation) -> DoorBundle {
        DoorBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(location.position().extend(2.)),
                sprite: Sprite {
                    anchor: location.anchor(),
                    color: constants::DOOR_COLOR,
                    custom_size: Some(Vec2::splat(DOOR_WIDTH)),
                    ..default()
                },
                ..default()
            },
        }
    }
}
