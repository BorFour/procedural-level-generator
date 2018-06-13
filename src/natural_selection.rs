use std::io::stdout;
use std::io::Write;
use std::cmp;


use rand;
use rand::distributions::Range;
use rand::prelude::*;

use rsgenetic::pheno::Phenotype;
use rsgenetic::sim::select::*;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::sim::*;

use generation;
use level::Level;
use position::search_path_in_level;
use position::Pos;
use room::Room;

static MUT_PROB: f64 = 0.85;
static CROSS_PROB: f64 = 1.00;
static N_ROOMS_PER_LEVEL: usize = 15;
static N_ELEMENTS_PER_ROOM: usize = 5;
static ZIP_CROSSOVER: bool = false;

impl Phenotype<i32> for Level {
    // How fit is this individual?
    fn fitness(&self) -> i32 {
        let result = search_path_in_level(&self);
        match result {
            Some(path) => self.get_rewards_from_path(&path.0)
                .iter()
                .fold(0, |a, acc| acc + a),
            None => -999999,
        }
    }

    // Have two individuals create a new individual
    fn crossover(&self, other: &Level) -> Level {
        let mut rng = rand::thread_rng();
        let mut cp = self.clone();

        let p: f64 = Range::new(0.0, 1.0).sample(&mut rng);

        if p < CROSS_PROB {
            // cp.name += "c";
            let mut new_rooms: Vec<Room> = Vec::new();
            if ZIP_CROSSOVER {
                for pair in self.rooms.iter().zip(&other.rooms) {
                    let x: usize = Range::new(0, 2).sample(&mut rng);
                    let room = if x == 0 {pair.0} else {pair.1};
                    new_rooms.push(room.clone());
                }
            } else {
                let bigger_size = cmp::max(self.rooms.len(), other.rooms.len());
                for i in 0..bigger_size {
                    let self_room = self.rooms.get(i);
                    let other_room = other.rooms.get(i);

                    if self_room.is_none() {
                        new_rooms.push(other_room.unwrap().clone());
                    } else if other_room.is_none() {
                        new_rooms.push(self_room.unwrap().clone());
                    } else {
                        let x: usize = Range::new(0, 2).sample(&mut rng);
                        let room = if x == 0 {self_room.unwrap()} else {other_room.unwrap()};
                        new_rooms.push(room.clone());
                    }
                }

            }
            cp.rooms = new_rooms;
        }
        return cp;
    }

    // Mutate an individual, changing its state
    fn mutate(&self) -> Level {
        // TODO: mutate by deleting or inserting a new room

        let mut rng = rand::thread_rng();
        let mut cp = self.clone();

        let p: f64 = Range::new(0.0, 1.0).sample(&mut rng);

        if p < MUT_PROB {
            // cp.name += "m";
            let x: usize = Range::new(0, self.rooms.len()).sample(&mut rng);
            cp.rooms[x].populate(N_ELEMENTS_PER_ROOM);
        }

        return cp;
    }
}

pub fn test_genetic(level: &Level) {
    let mut level2 = level.clone();

    for i in 0..level2.rooms.len() {
        // The lifetime of room only needs to be this long
        if let Some(r) = level2.rooms.get_mut(i) {
            r.elements = Vec::new();
            r.populate(N_ELEMENTS_PER_ROOM);
        }
    }

    println!("{:?}", level.fitness());
    level.show();
    level.crossover(&level).show(); // Crossover with itself should be the same
    level.crossover(&level2).show();
    level.mutate().show();
}

pub fn run_genertic_algorithm(iters: u64) {
    fn generate_individual() -> Level {
        let mut level = Level::new(
            "Level".to_owned(),
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

        level.rooms = generation::generate_rooms_in_level(&level, N_ROOMS_PER_LEVEL);

        for i in 0..level.rooms.len() {
            // The lifetime of room only needs to be this long
            if let Some(r) = level.rooms.get_mut(i) {
                r.populate(N_ELEMENTS_PER_ROOM);
            }
        }
        level
    }

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
        print!("{}", if i % 10 == 9 {"x"} else {"."});
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
}
