use crate::constants;
use bevy::prelude::*;

pub enum DoorLocation {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Component)]
pub struct Collider;

#[derive(Bundle)]
pub struct DoorBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

#[derive(Event, Default)]
pub struct CollisionEvent;

impl DoorLocation {
    fn position(&self) -> Vec2 {
        match self {
            DoorLocation::Left => {
                Vec2::new(constants::LEFT_WALL + constants::DOOR_THICKNESS / 2., 0.)
            }
            DoorLocation::Right => {
                Vec2::new(constants::RIGHT_WALL - constants::DOOR_THICKNESS / 2., 0.)
            }
            DoorLocation::Bottom => {
                Vec2::new(0., constants::BOTTOM_WALL + constants::DOOR_THICKNESS / 2.)
            }
            DoorLocation::Top => {
                Vec2::new(0., constants::TOP_WALL - constants::DOOR_THICKNESS / 2.)
            }
        }
    }
    fn size(&self) -> Vec2 {
        match self {
            DoorLocation::Right => Vec2::new(
                constants::WALL_THICKNESS + constants::DOOR_THICKNESS,
                constants::DOOR_WIDTH,
            ),
            DoorLocation::Left => Vec2::new(
                constants::WALL_THICKNESS + constants::DOOR_THICKNESS,
                constants::DOOR_WIDTH,
            ),
            DoorLocation::Top => Vec2::new(
                constants::DOOR_WIDTH,
                constants::WALL_THICKNESS + constants::DOOR_THICKNESS,
            ),
            DoorLocation::Bottom => Vec2::new(
                constants::DOOR_WIDTH,
                constants::WALL_THICKNESS + constants::DOOR_THICKNESS,
            ),
        }
    }
}

impl DoorBundle {
    pub fn new(location: DoorLocation) -> DoorBundle {
        let size = location.size();
        DoorBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: constants::DOOR_COLOR,
                    custom_size: Some(size.clone()),
                    ..default()
                },
                ..default()
            },

            collider: Collider,
        }
    }
}
