extern crate serde;
extern crate serde_json;

use types::RewardType;
// Elements like coins have a positive reward, and enemies or dangerous items have a negative one

pub trait HasReward {
    fn calculate_reward(&self) -> i32;
}

pub trait DesignElement {
    fn get_coordinates(&self) -> (u64, u64);
}

fn empty_fun() -> RewardType {
    return |_de: &GenericDesignElement| -> i32 { 0 };
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GenericDesignElement {
    pub name: String,
    pub x: u64,
    pub y: u64,
    // pub w: u64,
    // pub h: u64,
    #[serde(skip, default = "empty_fun")]
    pub _calculate_reward_fn: RewardType,
}

impl GenericDesignElement {
    pub fn get_element_names() -> Vec<String> {
        vec![
            "element".to_owned(),
            "chest".to_owned(),
            "coin".to_owned(),
            "hole".to_owned(),
            "foe".to_owned(),
            "boss".to_owned()
        ]
    }

    pub fn show(&self) {
        println!(
            "Reward for {:?}: {:?} in [{:?}, {:?}]",
            self.name,
            self.calculate_reward(),
            self.x,
            self.y
        );
    }

    pub fn new(name: String, x: u64, y: u64) -> GenericDesignElement {
        let reward_fn = reward_from_name(&name);
        GenericDesignElement {
            name: name,
            x: x,
            y: y,
            _calculate_reward_fn: reward_fn,
        }
    }
}

impl DesignElement for GenericDesignElement {
    fn get_coordinates(&self) -> (u64, u64) {
        return (self.x, self.y);
    }
}

impl HasReward for GenericDesignElement {
    fn calculate_reward(&self) -> i32 {
        let cr_fn = self._calculate_reward_fn;
        cr_fn(&self)
    }
}

pub fn reward_from_name(name: &String) -> RewardType {
    match name.as_ref() {
        "element" => |_de: &GenericDesignElement| -> i32 { 0 },
        "chest" => |_de: &GenericDesignElement| -> i32 { 25 },
        "coin" => |_de: &GenericDesignElement| -> i32 { 5 },
        "hole" => |_de: &GenericDesignElement| -> i32 { -5 },
        "foe" => |_de: &GenericDesignElement| -> i32 { -15 },
        "boss" => |_de: &GenericDesignElement| -> i32 { -40 },
        _ => |_de: &GenericDesignElement| -> i32 { 0 },
    }
}
