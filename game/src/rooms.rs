use crate::constants::{self, DOOR_WIDTH, TOP_WALL, WALL_WIDTH};
use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::{dynamics::RigidBody, geometry::Collider};

pub enum WallLocation {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Bundle)]
pub struct WallBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::TopLeft => Vec2::new(constants::LEFT_WALL, TOP_WALL),
            WallLocation::TopRight => Vec2::new(constants::RIGHT_WALL, TOP_WALL),
            WallLocation::BottomLeft => Vec2::new(constants::LEFT_WALL, constants::BOTTOM_WALL),
            WallLocation::BottomRight => Vec2::new(constants::RIGHT_WALL, constants::BOTTOM_WALL),
        }
    }

    fn anchor(&self) -> Anchor {
        match self {
            WallLocation::TopLeft => Anchor::TopLeft,
            WallLocation::TopRight => Anchor::TopRight,
            WallLocation::BottomLeft => Anchor::BottomLeft,
            WallLocation::BottomRight => Anchor::BottomRight,
        }
    }

    fn collider(&self) -> Collider {
        match self {
            WallLocation::BottomRight => Collider::polyline(
                vec![
                    Vec2::new(
                        -constants::WALL_WIDTH + DOOR_WIDTH / 2.,
                        constants::WALL_THICKNESS,
                    ),
                    Vec2::new(-constants::WALL_THICKNESS, constants::WALL_THICKNESS),
                    Vec2::new(
                        -constants::WALL_THICKNESS,
                        constants::WALL_HEIGHT - DOOR_WIDTH / 2.,
                    ),
                ],
                None,
            ),
            WallLocation::BottomLeft => Collider::polyline(
                vec![
                    Vec2::new(
                        constants::WALL_WIDTH - DOOR_WIDTH / 2.,
                        constants::WALL_THICKNESS,
                    ),
                    Vec2::new(constants::WALL_THICKNESS, constants::WALL_THICKNESS),
                    Vec2::new(
                        constants::WALL_THICKNESS,
                        constants::WALL_HEIGHT - DOOR_WIDTH / 2.,
                    ),
                ],
                None,
            ),
            WallLocation::TopRight => Collider::polyline(
                vec![
                    Vec2::new(
                        -constants::WALL_WIDTH + DOOR_WIDTH / 2.,
                        -constants::WALL_THICKNESS,
                    ),
                    Vec2::new(-constants::WALL_THICKNESS, -constants::WALL_THICKNESS),
                    Vec2::new(
                        -constants::WALL_THICKNESS,
                        -constants::WALL_HEIGHT + DOOR_WIDTH / 2.,
                    ),
                ],
                None,
            ),
            WallLocation::TopLeft => Collider::polyline(
                vec![
                    Vec2::new(WALL_WIDTH - DOOR_WIDTH / 2., -constants::WALL_THICKNESS),
                    Vec2::new(constants::WALL_THICKNESS, -constants::WALL_THICKNESS),
                    Vec2::new(
                        constants::WALL_THICKNESS,
                        -constants::WALL_HEIGHT + DOOR_WIDTH / 2.,
                    ),
                ],
                None,
            ),
        }
    }
}

impl WallBundle {
    pub fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.),
                    ..default()
                },
                sprite: Sprite {
                    color: constants::WALL_COLOR,
                    anchor: location.anchor(),
                    custom_size: Some(Vec2::new(constants::WALL_WIDTH, constants::WALL_HEIGHT)),
                    ..default()
                },
                ..default()
            },
            body: RigidBody::Fixed,
            collider: location.collider(),
        }
    }
}
