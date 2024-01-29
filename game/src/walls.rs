use crate::constants;
use bevy::prelude::*;

pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Bundle)]
pub struct WallBundle {
    sprite_bundle: SpriteBundle,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(constants::LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(constants::RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., constants::BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., constants::TOP_WALL),
        }
    }
    fn size(&self) -> Vec2 {
        let arena_height = constants::TOP_WALL - constants::BOTTOM_WALL;
        let arena_width = constants::RIGHT_WALL - constants::LEFT_WALL;

        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => Vec2::new(
                constants::WALL_THICKNESS,
                arena_height + constants::WALL_THICKNESS,
            ),
            WallLocation::Bottom | WallLocation::Top => Vec2::new(
                arena_width + constants::WALL_THICKNESS,
                constants::WALL_THICKNESS,
            ),
        }
    }
}

impl WallBundle {
    pub fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: constants::WALL_COLOR,
                    ..default()
                },
                ..default()
            },
        }
    }
}
