use std::collections::VecDeque;

use bevy::{
    app::{App, Plugin, PostUpdate, Update},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::{event_update_system, Event, EventReader, EventWriter},
        schedule::{common_conditions::in_state, Condition, IntoSystemConfigs},
        system::{Commands, Query},
    },
    math::{Quat, Vec2},
    prelude::SpatialBundle,
    transform::components::Transform,
};

use crate::{
    constants::{AppSet, AppState, GameState},
    scenes::console_log,
};

#[derive(Component)]
pub enum Inventory {
    OneHanded(Option<Entity>),
    DoubleHanded(Option<Entity>, Entity),
    Revolver(VecDeque<Entity>),
}
