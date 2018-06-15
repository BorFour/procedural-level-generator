use position::search_path_in_level;
use types::Constraint;

pub static NEUTRAL_CONS: Constraint = |_level| 0;
pub static TOO_SHORT_CONS: Constraint = |level| {
    let result = search_path_in_level(&level);
    if result.unwrap().0.len() < 4 {
        return -1000;
    }
    0
};

// pub fn less_than(n: usize) -> Constraint {
//     // TOO_SHORT_CONS
//     let m = n.clone();
//     let constraint: Constraint = |level| {
//         let result = search_path_in_level(&level);
//         if result.unwrap().0.len() < m {
//             return -1000;
//         }
//         0
//     };
//
//     return constraint;
// }
