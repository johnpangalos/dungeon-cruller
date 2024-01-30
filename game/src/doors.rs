use crate::constants::{self, BOTTOM_WALL, DOOR_WIDTH, LEFT_WALL, RIGHT_WALL, TOP_WALL};
use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::{
    dynamics::RigidBody,
    geometry::{Collider, Sensor},
};

#[derive(Component)]
pub enum Door {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Bundle)]
pub struct DoorBundle {
    door: Door,
    sensor: Sensor,
    body: RigidBody,
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl Door {
    fn position(&self) -> Vec2 {
        let wall_position = match self {
            Door::Left => Vec2::new(LEFT_WALL, 0.),
            Door::Right => Vec2::new(RIGHT_WALL, 0.),
            Door::Bottom => Vec2::new(0., BOTTOM_WALL),
            Door::Top => Vec2::new(0., TOP_WALL),
        };
        wall_position
    }

    fn anchor(&self) -> Anchor {
        match self {
            Door::Left => Anchor::CenterLeft,
            Door::Right => Anchor::CenterRight,
            Door::Bottom => Anchor::BottomCenter,
            Door::Top => Anchor::TopCenter,
        }
    }
}

impl DoorBundle {
    pub fn new(location: Door) -> DoorBundle {
        let transform = Transform::from_translation(location.position().extend(2.));
        let anchor = location.anchor();

        DoorBundle {
            door: location,
            sensor: Sensor,
            sprite_bundle: SpriteBundle {
                transform,
                sprite: Sprite {
                    anchor,
                    color: constants::DOOR_COLOR,
                    custom_size: Some(Vec2::splat(DOOR_WIDTH)),
                    ..default()
                },
                ..default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(DOOR_WIDTH / 2., DOOR_WIDTH / 2.),
        }
    }
}
