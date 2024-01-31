use crate::{
    items::components::{Bullet, Lifetime},
    player::Speed,
};
use bevy::{
    ecs::{
        component::Component,
        event::EventReader,
        system::{Commands, Query},
    },
    math::{Quat, Vec2},
    prelude::SpatialBundle,
    transform::components::Transform,
};

use super::components::ItemEvent;

#[derive(Component)]
pub struct CasualBulletItem;

pub fn use_item(
    mut commands: Commands,
    mut reader: EventReader<ItemEvent>,
    query: Query<&CasualBulletItem>,
) {
    fn item_used(commands: &mut Commands, position: &Vec2, direction: &Vec2) {
        commands.spawn((
            Bullet,
            SpatialBundle::from_transform(Transform {
                translation: position.extend(0.),
                rotation: Quat::from_rotation_z(direction.angle_between(Vec2::X)),
                ..Transform::default()
            }),
            Lifetime(2.0),
            Speed(10.0),
        ));
    }

    for event in reader.read() {
        if let ItemEvent::Used {
            entity,
            position,
            direction,
        } = event
        {
            if let Ok(CasualBulletItem) = query.get(*entity) {
                item_used(&mut commands, position, direction)
            }
        }
    }
}
