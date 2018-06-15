use design_element::GenericDesignElement;
use room_level::RoomLevel;

pub type Location = (u64, u64);
pub type RewardType = fn(&GenericDesignElement) -> i32;
pub type Constraint = fn(&RoomLevel) -> i32;
