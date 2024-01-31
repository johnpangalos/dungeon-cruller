mod casual_bullet_item;
pub mod components;
mod console_item;

use bevy::{
    app::{App, FixedUpdate, Plugin, Update},
    ecs::{
        entity::Entity,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res},
    },
    hierarchy::DespawnRecursiveExt,
    time::Time,
    transform::components::Transform,
};
pub use casual_bullet_item::CasualBulletItem;
pub use console_item::ConsoleItem;

use crate::constants::AppSet;

use self::components::{ItemEvent, Lifetime, Trajectory};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemEvent>()
            .add_systems(FixedUpdate, (move_bullet, kill_bullet))
            .add_systems(
                Update,
                (console_item::use_item, casual_bullet_item::use_item)
                    .in_set(AppSet::Items)
                    .after(AppSet::Player),
            );
    }
}

fn move_bullet(
    mut query: Query<(&mut Transform, &Trajectory, &Lifetime), With<components::Bullet>>,
) {
    for (mut transform, Trajectory(points), lifetime) in &mut query {
        let (mut prev, mut next) = (None, None);

        let t = lifetime.current / lifetime.lifespan;

        for point in points {
            prev = next;
            next = Some(point);

            if t <= point.1 {
                break;
            }
        }

        println!("{:?} {:?} {:?}", t, prev, next);

        if let (Some(from), Some(to)) = (prev, next) {
            let time_difference = to.1 - from.1;

            let progress = (t - from.1) / time_difference;

            let position = from.0.lerp(to.0, progress);

            transform.translation.x = position.x;
            transform.translation.y = position.y;
        }
    }
}

fn kill_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime), With<components::Bullet>>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.current += time.delta_seconds();
        if lifetime.current >= lifetime.lifespan {
            commands.entity(entity).despawn_recursive();
        }
    }
}
