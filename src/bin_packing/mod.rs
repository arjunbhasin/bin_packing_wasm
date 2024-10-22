pub mod bin;
pub mod item;
pub mod order;
pub mod solver;

use bin::Bin;

pub fn sort_bin_list_by_weight(bin_list: &mut Vec<Bin>, ascending: bool) {
    match ascending {
        true => bin_list.sort_by(|a, b| a.max_weight.partial_cmp(&b.max_weight).unwrap()),
        false => bin_list.sort_by(|a, b| b.max_weight.partial_cmp(&a.max_weight).unwrap()),
    }
}
