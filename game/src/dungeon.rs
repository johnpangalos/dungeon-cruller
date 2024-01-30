use crate::rooms::Room;
use bevy::prelude::*;

#[derive(Component)]
pub struct Dungeon {
    pub layout: Vec<Room>,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        let layout = vec![
            Room::new(1, "textures/wooden-floor.png".to_string()),
            Room::new(2, "textures/wooden-floor.png".to_string()),
        ];
        return Dungeon { layout };
    }
}
