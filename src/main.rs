#[macro_use]
extern crate serde_derive;
extern crate pathfinding;
extern crate rand;
extern crate rsgenetic;

mod level_rooms;
mod generation;
mod level;
mod natural_selection;
mod position;
mod types;

use level_rooms::*;

fn main() {
    // natural_selection::test_genetic(&level1);
    let generated_level: room_level::RoomLevel = natural_selection::run_genetic_algorithm(200);
    match generated_level.save_to_file() {
        Ok(_) => println!("File saved sucessfully"),
        Err(_) => println!("Error saving file"),
    }

    if let Some(solution) = position::search_path_in_level(&generated_level) {
        println!("Path from start to exit: {:?}", solution.0);
        for pos in solution.0 {
            generated_level.get_room_from_position(&pos).unwrap().show();
        }
    }
}
