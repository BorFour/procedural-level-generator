use design_element::GenericDesignElement;

pub type Location = (u64, u64);
pub type RewardType = fn(&GenericDesignElement) -> i32;
