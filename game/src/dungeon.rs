use crate::rooms::Room;
use bevy::prelude::*;

#[derive(Component)]
pub struct Dungeon {
    pub layout: Vec<Room>,
    pub current_room_x: isize,
    pub current_room_y: isize,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        let layout = vec![
            Room::new(0, 0, "textures/wooden-floor.png".to_string()),
            Room::new(0, 1, "textures/wooden-floor.png".to_string()),
        ];
        return Dungeon {
            layout,
            current_room_x: 0,
            current_room_y: 0,
        };
    }

    pub fn get_current_room(&self) -> Option<&Room> {
        return self.layout.iter().find(|room| {
            room.coord_x == self.current_room_x && room.coord_y == self.current_room_y
        });
    }
}
