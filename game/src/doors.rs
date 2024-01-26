use crate::constants;
use crate::player;
use crate::scenes::console_log;
use bevy::render::primitives::Aabb;
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
    // aabb: Aabb,
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

pub fn check_door_collisions(
    _commands: Commands,
    player_query: Query<(&Transform, &Aabb), With<player::Player>>,
    collider_query: Query<(&Transform, &Aabb), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (player_transform, player_aabb) = player_query.single();

    console_log(
        "player position",
        format!("{:?}", player_transform.translation),
    );
    console_log(
        "player size",
        format!("{:?}", player_transform.scale.truncate()),
    );

    for (i, (transform, aabb)) in collider_query.iter().enumerate() {
        console_log(
            format!("transform position {}", i),
            format!("{:?}", aabb.center),
        );
        console_log(
            format!("transform size {}", i),
            format!("{:?}", aabb.half_extents),
        );

        let collision = collide(
            player_transform.translation + Vec3::from(player_aabb.center),
            player_aabb.half_extents.truncate() * Vec2::splat(2.),
            transform.translation + Vec3::from(aabb.center),
            aabb.half_extents.truncate() * Vec2::splat(2.),
        );

        if let Some(_collision) = collision {
            collision_events.send_default();
        }
    }
}

pub fn print_collision(_commands: Commands, mut collision_events: EventReader<CollisionEvent>) {
    if !collision_events.is_empty() {
        collision_events.clear();
        console_log("collision", "done");
    }
}
