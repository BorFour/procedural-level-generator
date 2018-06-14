extern crate serde;
extern crate serde_json;

use types::Location;
use generation;
use design_element;
use design_element::HasReward;
use position::Pos;

#[derive(Serialize, Deserialize, Clone)]
// #[derive(Clone, Serialize, Deserialize, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Room {
    pub name: String,
    pub position: Pos,
    pub elements: Vec<design_element::GenericDesignElement>,
    pub w: u64,
    pub h: u64,
}


impl Room {
    pub fn new(name: String, position: Pos, elements: Vec<design_element::GenericDesignElement>) -> Room {
        Room {
            name: name,
            position: position,
            elements: elements,
            w: 50,
            h: 50,
        }
    }

    pub fn random_location(&self) -> Location {
        generation::random_coordinates((0, self.w), (0, self.h))
    }

    pub fn show(&self) {
        println!(
            "Reward for room {:?}: {:?}",
            self.name,
            self.calculate_reward()
        );
    }

    //TODO: add constraints to the population.
    pub fn populate(&mut self, n_elements: usize) {
        for _i in 0..n_elements {
            let element = generation::new_element_in_room(&self);
            self.elements.push(element);
        }
    }
}

impl HasReward for Room {
    fn calculate_reward(&self) -> i32 {
        let mut result = 0;
        for element in &self.elements {
            result += element.calculate_reward();
        }
        return result;
    }
}
