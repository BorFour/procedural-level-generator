#[macro_use]
extern crate serde_derive;
// This dependency needs to be here and it doesn't compile if you put it in design_element.rs
// Why??
extern crate pathfinding;
extern crate rand;
extern crate rsgenetic;

// use std::collections::HashMap;
mod design_element;
mod generation;
mod level;
mod natural_selection;
mod position;
mod room;
mod room_level;
mod types;

fn main() {
    // natural_selection::test_genetic(&level1);
    let generated_level: room_level::RoomLevel = natural_selection::run_genetic_algorithm(200, room_level::generate_individual);
    match generated_level.save_to_file() {
        Ok(_) => println!("File saved sucessfully"),
        Err(_) => println!("Error saving file"),
    }
}
