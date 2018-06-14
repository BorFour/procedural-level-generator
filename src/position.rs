// use pathfinding::bfs::*;
use pathfinding::prelude::{astar};

use room_level::RoomLevel;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    #[allow(unused)]
    pub fn neighbours(&self) -> Vec<Pos> {
        let &Pos(x, y) = self;
        vec![Pos(x - 1, y), Pos(x + 1, y), Pos(x, y - 1), Pos(x, y + 1)]
    }

    pub fn neighbours_with_size(&self) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        vec![Pos(x - 1, y), Pos(x + 1, y), Pos(x, y - 1), Pos(x, y + 1)]
            .into_iter()
            .map(|p| (p, 1))
            .collect()
    }
    //
    // fn distance(&self, other: &Pos) -> i32 {
    //     (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as f64).sqrt() as i32
    // }

    fn distance(&self, other: &Pos) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
    }

    fn is_equal(&self, &other: &Pos) -> bool {
        (self.0 == other.0) && (self.1 == other.1)
    }
}

#[allow(unused)]
pub fn search_path() {
    static GOAL: Pos = Pos(4, 14);
    let result = astar(
        &Pos(1, 0),
        // |p| Vec::new(),
        |p| p.neighbours_with_size(),
        |p| p.distance(&GOAL) / 3,
        |p| *p == GOAL,
    );
    // let result = bfs(&Pos(1, 0), |p| p.neighbours(), |p| *p == GOAL);
    if let Some(path) = result {
        println!("Path found: {:?}", path);
    } else {
        println!("No path found");
    }
}

pub fn search_path_in_level(level: &RoomLevel) -> Option<(Vec<Pos>, usize)> {
    let start: Pos = level.rooms[0].position.clone();
    let goal: Pos = level.rooms[level.rooms.len() - 1].position.clone();
    // println!("Start: {:?}", start);
    // println!("Goal: {:?}", goal);
    astar(
        &start,
        |p| level.neighbours_from_position(*p),
        |p| p.distance(&goal) / 3,
        |p| p.is_equal(&goal),
    )
}
