// use std::cmp;
use std::io::stdout;
use std::io::Write;

// use rand;
// use rand::distributions::Range;
// use rand::prelude::*;

use rsgenetic::pheno::Phenotype;
use rsgenetic::sim::select::*;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::sim::*;

// use generation;
use position::search_path_in_level;
// use position::Pos;
// use room::Room;
use room_level::RoomLevel;

#[allow(unused)]
pub fn test_genetic(level: &RoomLevel) {
    let mut level2 = level.clone();

    for i in 0..level2.rooms.len() {
        // The lifetime of room only needs to be this long
        if let Some(r) = level2.rooms.get_mut(i) {
            r.elements = Vec::new();
            r.populate(10);
        }
    }

    println!("{:?}", level.fitness());
    level.show();
    level.crossover(&level).show(); // Crossover with itself should be the same
    level.crossover(&level2).show();
    level.mutate().show();
}

pub fn run_genetic_algorithm(iters: u64, generate_individual: fn() -> RoomLevel) -> RoomLevel {

    let population_size: usize = 200;
    let mut population = (0..population_size)
        .map(|_i| generate_individual())
        .collect();
    let mut s = Simulator::builder(&mut population)
                    .set_selector(Box::new(TournamentSelector::new(population_size/5, 10)))
                    // .set_selector(Box::new(StochasticSelector::new(10)))
                    .set_max_iters(iters)
                    .build();
    println!("Running {:?} iterations of the genetic algorithm", iters);
    println!("Population size: {:?} individuals", population_size);
    // s.run();
    for i in 0..iters {
        s.checked_step();
        print!("{}", if i % 10 == 9 { "x" } else { "." });
        stdout().flush();
    }
    println!();

    let result = s.get().unwrap(); // The best individual
    result.show();
    println!("Fitness: {:?}", result.fitness());
    // for room in &result.rooms {
    //     room.show();
    //     // println!("{:?}", room.position);
    // }
    if let Some(solution) = search_path_in_level(&result) {
        println!("Path from start to exit: {:?}", solution.0);
        for pos in solution.0 {
            result.get_room_from_position(&pos).unwrap().show();
        }
    }
    return result.clone();
}
