use crate::rooms::Room;

pub fn get_dungeon() -> Box<[Room]> {
    return Box::new([Room::new(1), Room::new(2)]);
}
