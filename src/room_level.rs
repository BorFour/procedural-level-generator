extern crate serde;
extern crate serde_json;

use std::cmp;
use std::fs::File;
use std::io::Result;

use rand;
use rand::distributions::Range;
use rand::prelude::*;

use rsgenetic::pheno::Phenotype;

use generation;
use design_element::HasReward;
use level::Level;
use position::search_path_in_level;
use position::Pos;
use room;
use room::Room;

static MUT_PROB: f64 = 0.85;
static CROSS_PROB: f64 = 1.00;
static ZIP_CROSSOVER: bool = false;
static N_ROOMS_PER_LEVEL: usize = 15;
static N_ELEMENTS_PER_ROOM: usize = 5;

#[derive(Serialize, Deserialize, Clone)]
pub struct RoomLevel {
    pub name: String,
    pub rooms: Vec<room::Room>,
    pub w: u64,
    pub h: u64,
}

impl RoomLevel {
    pub fn new(name: String, rooms: Vec<room::Room>) -> RoomLevel {
        RoomLevel {
            name: name,
            rooms: rooms,
            w: 6,
            h: 4,
        }
    }

    pub fn show(&self) {
        print!(
            "Reward for RoomLevel {:?}: {:?}",
            self.name,
            self.calculate_reward()
        );
        for room in &self.rooms {
            print!(" {:?}", room.position);
        }
        println!();
    }

    pub fn save_to_file(&self) -> Result<()> {
        let f = File::create("foo.json")?;
        serde_json::to_writer_pretty(&f, &self).unwrap();
        f.sync_data()?;
        Ok(())
    }

    pub fn get_room_from_position(&self, position: &Pos) -> Option<room::Room> {
        for room in &self.rooms {
            let p = room.position;
            if p.0 == position.0 && p.1 == position.1 {
                return Some(room.clone());
            }
        }
        None
    }

    pub fn neighbours_from_position(&self, position: Pos) -> Vec<(Pos, usize)> {
        let mut result = Vec::new();

        if let Some(r) = self.get_room_from_position(&Pos(position.0 - 1, position.1)) {
            result.push(r);
        }

        if let Some(r) = self.get_room_from_position(&Pos(position.0 + 1, position.1)) {
            result.push(r);
        }

        if let Some(r) = self.get_room_from_position(&Pos(position.0, position.1 - 1)) {
            result.push(r);
        }

        if let Some(r) = self.get_room_from_position(&Pos(position.0, position.1 + 1)) {
            result.push(r);
        }

        result.into_iter().map(|p| (p.position, 1)).collect()
    }

    pub fn get_rewards_from_path(&self, path: &Vec<Pos>) -> Vec<i32> {
        path.into_iter()
            .map(|p| self.get_room_from_position(&p))
            .map(|option| match option {
                Some(room) => room.calculate_reward(),
                None => 0,
            })
            .collect()
    }
}

impl Phenotype<i32> for RoomLevel {
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
    fn crossover(&self, other: &RoomLevel) -> RoomLevel {
        let mut rng = rand::thread_rng();
        let mut cp = self.clone();

        let p: f64 = Range::new(0.0, 1.0).sample(&mut rng);

        if p < CROSS_PROB {
            // cp.name += "c";
            let mut new_rooms: Vec<Room> = Vec::new();
            if ZIP_CROSSOVER {
                for pair in self.rooms.iter().zip(&other.rooms) {
                    let x: usize = Range::new(0, 2).sample(&mut rng);
                    let room = if x == 0 {
                        pair.0
                    } else {
                        pair.1
                    };
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
                        let room = if x == 0 {
                            self_room.unwrap()
                        } else {
                            other_room.unwrap()
                        };
                        new_rooms.push(room.clone());
                    }
                }
            }
            cp.rooms = new_rooms;
        }
        return cp;
    }

    // Mutate an individual, changing its state
    fn mutate(&self) -> RoomLevel {
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

impl Level for RoomLevel {}

impl HasReward for RoomLevel {
    fn calculate_reward(&self) -> i32 {
        return self.rooms
            .iter()
            .map(|r| r.calculate_reward())
            .fold(0, |acc, len| acc + len);
    }
}

pub fn generate_individual() -> RoomLevel {
    let mut level = RoomLevel::new(
        "RoomLevel".to_owned(),
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
