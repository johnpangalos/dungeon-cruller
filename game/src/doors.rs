use crate::constants;
use crate::player;
use crate::scenes::console_log;
use bevy::{prelude::*, sprite::collide_aabb::collide};

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
        DoorBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: constants::DOOR_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

pub fn check_door_collisions(
    _commands: Commands,
    player_query: Query<&Transform, With<player::Player>>,
    collider_query: Query<&Transform, With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let player_transform = player_query.single();

    for transform in &collider_query {
        console_log(
            "player position",
            format!("{:?}", player_transform.translation),
        );
        console_log(
            "player size",
            format!("{:?}", player_transform.scale.truncate()),
        );
        console_log("transform position", format!("{:?}", transform.translation));
        console_log(
            "transform size",
            format!("{:?}", transform.scale.truncate()),
        );

        let collision = collide(
            player_transform.translation,
            player_transform.scale.truncate(),
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(_collision) = collision {
            collision_events.send_default();
        }
    }
}

pub fn print_collision(_commands: Commands, mut collision_events: EventReader<CollisionEvent>) {
    if !collision_events.is_empty() {
        collision_events.clear();
        println!("collide!")
    }
}
