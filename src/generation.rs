use rand;
use rand::distributions::Range;
use rand::prelude::*;

use design_element::GenericDesignElement;
use position::Pos;
use room::Room;
use room_level::RoomLevel;
use types::Location;

// Generation functions

#[allow(unused)]
pub fn generate_rooms_from_positions(positions: Vec<Pos>) -> Vec<Room> {
    positions
        .into_iter()
        .map(|p| Room::new("Room".to_owned(), p, Vec::new()))
        .collect()
}

pub fn random_coordinates(range_x: (u64, u64), range_y: (u64, u64)) -> Location {
    let mut rng = rand::thread_rng();
    let x: u64 = Range::new(range_x.0, range_x.1).sample(&mut rng);
    let y: u64 = Range::new(range_y.0, range_y.1).sample(&mut rng);
    return (x, y);
}

pub fn new_random_design_element(x: u64, y: u64) -> GenericDesignElement {
    let names = GenericDesignElement::get_element_names();
    let name = rand::thread_rng().choose(&names);
    match name {
        Some(thing) => GenericDesignElement::new(thing.to_owned(), x, y),
        _ => GenericDesignElement::new("default".to_owned(), x, y),
    }
}

pub fn new_element_in_room(room: &Room) -> GenericDesignElement {
    let location = room.random_location();
    new_random_design_element(location.0, location.1)
}

pub fn generate_rooms_in_level(level: &RoomLevel, n_rooms: usize) -> Vec<Room> {
    let mut rooms: Vec<Room> = Vec::new();
    for _ in 0..n_rooms {
        loop {
            let coordinates: Location = random_coordinates((0, level.w), (0, level.h));
            let position: Pos = Pos(coordinates.0 as i32, coordinates.1 as i32);
            if let Some(_room) = level.get_room_from_position(&position) {
                continue;
            }
            rooms.push(Room::new("Room".to_owned(), position, Vec::new()));
            break;
        }
    }

    return rooms;
}
