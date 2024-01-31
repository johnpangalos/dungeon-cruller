use crate::items::components::{Bullet, Lifetime, Trajectory};
use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        component::Component,
        event::EventReader,
        system::{Commands, Query, ResMut},
    },
    math::{
        cubic_splines::{CubicBSpline, CubicBezier, CubicCurve, CubicGenerator, CubicSegment},
        Quat, Vec2, Vec3,
    },
    prelude::default,
    render::texture::Image,
    sprite::{self, Sprite, SpriteBundle},
    transform::components::Transform,
};
use bevy_rapier2d::{
    dynamics::RigidBody,
    geometry::{Collider, Sensor},
};

use super::components::ItemEvent;

#[derive(Component)]
pub struct CasualBulletItem;

pub fn use_item(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut reader: EventReader<ItemEvent>,
    query: Query<&CasualBulletItem>,
) {
    fn item_used(
        commands: &mut Commands,
        position: &Vec2,
        rotation: &Quat,
        texture: Handle<Image>,
    ) {
        let sprite_bundle = SpriteBundle {
            transform: Transform {
                translation: position.extend(2.),
                rotation: rotation.clone(),
                ..default()
            },
            texture,
            sprite: Sprite {
                custom_size: Some(Vec2::new(64., 64.)),
                ..default()
            },
            ..default()
        };

        commands.spawn((
            Bullet,
            RigidBody::KinematicPositionBased,
            Sensor,
            Collider::cuboid(0.5, 0.5),
            Trajectory::straight(
                position.clone(),
                sprite_bundle.transform.right().truncate(),
                1000.,
            ),
            sprite_bundle,
            Lifetime {
                current: 0.,
                lifespan: 2.,
            },
        ));
    }

    let bullet = asset_server.load::<Image>("textures/bullet.png");

    for event in reader.read() {
        if let ItemEvent::Used {
            entity,
            position,
            rotation,
        } = event
        {
            if let Ok(CasualBulletItem) = query.get(*entity) {
                item_used(&mut commands, position, rotation, bullet.clone());
            }
        }
    }
}
