extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::Result;

use rsgenetic::pheno::*;

use room;
use design_element::HasReward;
use position::Pos;

#[derive(Serialize, Deserialize, Clone)]
pub struct Level {
    pub name: String,
    pub rooms: Vec<room::Room>,
    pub w: u64,
    pub h: u64
}

impl Level {
    pub fn new(name: String, rooms: Vec<room::Room>) -> Level {
        Level {
            name: name,
            rooms: rooms,
            w: 6,
            h: 4
        }
    }

    pub fn show(&self) {
        print!(
            "Reward for level {:?}: {:?}",
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
        let room = self.get_room_from_position(&position);
        let mut result = Vec::new();

        if let Some(r) = self.get_room_from_position(&Pos(position.0-1, position.1)) {
            result.push(r);
        }

        if let Some(r) = self.get_room_from_position(&Pos(position.0+1, position.1)) {
            result.push(r);
        }

        if let Some(r) = self.get_room_from_position(&Pos(position.0, position.1-1)) {
            result.push(r);
        }

        if let Some(r) = self.get_room_from_position(&Pos(position.0, position.1+1)) {
            result.push(r);
        }

        result.into_iter().map(|p| (p.position, 1)).collect()
    }

    pub fn get_rewards_from_path(&self, path: &Vec<Pos>) -> Vec<i32> {
        path.into_iter()
        .map(|p| self.get_room_from_position(&p))
        .map(|option| {
            match option {
                Some(room) => room.calculate_reward(),
                None => 0
            }
        })
        .collect()
    }
}

impl HasReward for Level {
    fn calculate_reward(&self) -> i32 {
        return self.rooms
            .iter()
            .map(|r| r.calculate_reward())
            .fold(0, |acc, len| acc + len);
    }
}
