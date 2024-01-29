use crate::constants::{self, TOP_WALL};
use bevy::{prelude::*, sprite::Anchor};

pub enum WallLocation {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Bundle)]
pub struct WallBundle {
    sprite_bundle: SpriteBundle,
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
}

impl WallBundle {
    pub fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(-2.),
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
        }
    }
}
