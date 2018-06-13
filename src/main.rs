#[macro_use]
extern crate serde_derive;
// This dependency needs to be here and it doesn't compile if you put it in design_element.rs
// Why??
extern crate pathfinding;
extern crate rsgenetic;
extern crate rand;

// use std::collections::HashMap;
mod design_element;
mod generation;
mod level;
mod position;
mod room;
mod types;
mod natural_selection;

// use design_element::HasReward;
use design_element::GenericDesignElement;
use level::Level;
use position::Pos;
use room::Room;

fn main() {
    // let elements_data = vec![("element", 0, 5), ("coin", 2, 3), ("foe", 1, 1)];
    //
    // let mut room1 = Room::new("Room number 1".to_owned(), Pos(0, 0), Vec::new());
    //
    // for data in elements_data {
    //     room1
    //         .elements
    //         .push(GenericDesignElement::new(data.0.to_owned(), data.1, data.2));
    // }

    let mut level1 = Level::new(
        "Level 1".to_owned(),
        Vec::new()
        // generation::generate_rooms_from_positions(vec![
        //     Pos(1, 0), Pos(2, 0), Pos(2, 1), Pos(2, 2), Pos(2, 3), Pos(3, 3), Pos(3, 2), Pos(3, 1),
        //     Pos(4, 1), Pos(4, 0), Pos(4, 2)
        // ])
        // vec![
        //     room1,
        //     Room::new("Room number 2".to_owned(), Pos(1, 0), Vec::new()),
        //     Room::new("Room number 3".to_owned(), Pos(2, 0), Vec::new()),
        //     Room::new("Room number 4".to_owned(), Pos(3, 0), Vec::new()),
        //     Room::new("Room number 5".to_owned(), Pos(4, 0), Vec::new()),
        // ],
    );

    level1.rooms = generation::generate_rooms_in_level(&level1, 25);

    for i in 0..level1.rooms.len() {
        // The lifetime of room only needs to be this long
        if let Some(r) = level1.rooms.get_mut(i) {
            r.populate(5);
            // r.show();
        }
    }

    // for de in &level1.rooms[0].elements {
    //     de.show();
    // }

    level1.rooms[0].show();
    level1.show();

    match level1.save_to_file() {
        Ok(_) => println!("File saved sucessfully"),
        Err(_) => println!("Error saving file"),
    }

    /*
    position::search_path();
    let result = position::search_path_in_level(&level1);
    if let Some(path) = result {
        // println!("Path: {:?}", path);
        let rewards = level1.get_rewards_from_path(&path.0);
        {
            // Not necessary to create a new scope but a good practice imo
            let zipped: Vec<_> = path.0.iter().zip(&rewards).collect();
            println!("Rewards per room {:?}", zipped);
        }
        println!("{:?}", rewards.iter().fold(0, |a, acc| acc + a));
    } else {
         println!("No path available")
    }
    */

    // natural_selection::test_genetic(&level1);
    natural_selection::run_genertic_algorithm(200);


}
