/*
    This solver is modified to be used with the wasm_bindgen library.
    Modified functions:
    - get_smallest_fitting_bin_for_item_vector
*/
pub mod knapsack;
use crate::bin_packing::bin::Bin;
use crate::bin_packing::item::Item;
use js_sys::Function;
use wasm_bindgen::JsValue;

// "Axis-Aligned Bounding Box" (AABB) collision detection method
fn check_rectangle_intersection(
    existing_item_dimension: &[f32; 3],
    existing_item_position: &[f32; 3],
    new_item_dimension: &[f32; 3],
    new_item_position: &[f32; 3],
    x: usize,
    y: usize,
) -> bool {
    let existing_min_x = existing_item_position[x];
    let existing_max_x = existing_item_position[x] + existing_item_dimension[x];
    let existing_min_y = existing_item_position[y];
    let existing_max_y = existing_item_position[y] + existing_item_dimension[y];

    let new_min_x = new_item_position[x];
    let new_max_x = new_item_position[x] + new_item_dimension[x];
    let new_min_y = new_item_position[y];
    let new_max_y = new_item_position[y] + new_item_dimension[y];

    existing_min_x < new_max_x
        && existing_max_x > new_min_x
        && existing_min_y < new_max_y
        && existing_max_y > new_min_y
}

fn check_intersection(
    existing_item: &Item,
    new_item_dimension: &[f32; 3],
    new_item_position: &[f32; 3],
) -> bool {
    let existing_dimension = existing_item.get_rotated_dimension(&existing_item.rotation);
    let existing_position = &existing_item.position;

    if !check_rectangle_intersection(
        &existing_dimension,
        &existing_position,
        &new_item_dimension,
        &new_item_position,
        0,
        1,
    ) {
        return false;
    }
    if !check_rectangle_intersection(
        &existing_dimension,
        &existing_position,
        &new_item_dimension,
        &new_item_position,
        1,
        2,
    ) {
        return false;
    }
    if !check_rectangle_intersection(
        &existing_dimension,
        &existing_position,
        &new_item_dimension,
        &new_item_position,
        0,
        2,
    ) {
        return false;
    }
    true
}

fn check_item_in_bin_at_pivot(
    bin: &Bin,
    item: &Item,
    pivot: &[f32; 3],
    orientable: bool,
) -> Option<String> {
    let rotations = if orientable {
        vec!["RT_WHD", "RT_HWD", "RT_HDW", "RT_DHW", "RT_DWH", "RT_WDH"]
    } else {
        vec!["RT_WHD", "RT_DWH"]
    };

    for rt in rotations {
        let item_dimension = item.get_rotated_dimension(rt);

        // Check if item at pivot exceeds bin dimensions, if yes then try next rotation
        if pivot[0] + item_dimension[0] > bin.width
            || pivot[1] + item_dimension[1] > bin.height
            || pivot[2] + item_dimension[2] > bin.depth
        {
            continue;
        }

        // item at current rotation, doesn't exceed bin dimensions
        // for each item in bin, check if there is an intersection

        let mut intersection_failure = false;

        for item_in_bin in &bin.packed_items {
            if check_intersection(&item_in_bin, &item_dimension, pivot) {
                intersection_failure = true;
                break; //exit this for loop
            }
        }

        if !intersection_failure {
            return Some(rt.to_string());
        }
    }
    // by default return false
    return None;
}

pub fn pack_item_to_bin(
    bin: &mut Bin,
    item: &Item,
    open_pivots: &[[f32; 3]],
    stackable: bool,
    orientable: bool,
) -> Option<Vec<[f32; 3]>> {
    for pivot in open_pivots {
        // Check volume and weight constraints
        if item.get_volume() + bin.get_packed_items_volume() > bin.get_volume()
            || item.weight + bin.get_packed_items_weight() > bin.max_weight
        {
            // Item cannot be packed into this bin due to volume or weight constraints
            return None;
        }

        if let Some(rotation_string) = check_item_in_bin_at_pivot(bin, item, pivot, orientable) {
            // Get the rotated dimensions using the rotation_string
            let item_dimension = item.get_rotated_dimension(&rotation_string);

            // Initialize new pivots
            let mut new_pivots = Vec::new();

            // Create new pivots based on the current pivot and item dimensions
            let new_pivot_along_width = [pivot[0] + item_dimension[0], pivot[1], pivot[2]];
            let new_pivot_along_depth = [pivot[0], pivot[1], pivot[2] + item_dimension[2]];
            let new_pivot_along_height = [pivot[0], pivot[1] + item_dimension[1], pivot[2]];

            // Check if the new pivots are within bin dimensions before adding them
            if new_pivot_along_width[0] <= bin.width {
                new_pivots.push(new_pivot_along_width);
            }
            if new_pivot_along_depth[2] <= bin.depth {
                new_pivots.push(new_pivot_along_depth);
            }
            if stackable && new_pivot_along_height[1] <= bin.height {
                new_pivots.push(new_pivot_along_height);
            }

            // Copy existing open pivots except the used one
            for &existing_pivot in open_pivots {
                if existing_pivot != *pivot {
                    new_pivots.push(existing_pivot);
                }
            }

            // Now create packed_item and push it into bin.packed_items
            let mut packed_item = item.clone();
            packed_item.rotation = rotation_string;
            packed_item.position = *pivot;
            bin.packed_items.push(packed_item);

            return Some(new_pivots);
        }
    }
    None // Item cannot be packed into the bin at any pivot
}

pub fn get_smallest_fitting_bin_for_item_vector(
    sorted_item_list: &Vec<Item>,
    sorted_bin_list: &mut Vec<Bin>,
    total_order_weight: f32,
    total_order_volume: f32,
    js_update_function: &Function,
) -> Option<Bin> {
    for (bin_index, bin) in sorted_bin_list.iter_mut().enumerate() {
        // Reject bin if orders don't pass basic W/V tests
        if bin.max_weight < total_order_weight || bin.get_volume() < total_order_volume {
            continue;
        }
        // Initialize open pivots
        let mut open_pivots = vec![[0.0, 0.0, 0.0]];

        for (item_index, item) in sorted_item_list.iter().enumerate() {
            if let Some(new_pivots) = pack_item_to_bin(
                bin,
                item,
                &open_pivots,
                match item.stackable {
                    Some(s) => s,
                    None => true, // default to stackable
                },
                match item.orientable {
                    Some(o) => o,
                    None => true, // default to orientable
                },
            ) {
                open_pivots = new_pivots;
                if item_index % 10 == 0 {
                    let progress_string: String = format!("{}-{}", bin_index, item_index);
                    // pass progress to js
                    _ = js_update_function.call1(&JsValue::NULL, &JsValue::from(progress_string));
                }
            }
        }

        // If all items are packed
        if bin.packed_items.len() == sorted_item_list.len() {
            return Some(bin.clone());
        }
    }
    None
}

// pub fn get_smallest_fitting_bin_for_order_vector(
//     order_list: Vec<Order>,
//     bin_list: &mut Vec<Bin>,
// ) -> Option<Bin> {
//     for bin in bin_list {
//         // TODO: reject bin if orders don't pass basic W/V tests

//         // Use iterator magic to efficiently gather all items to pack
//         let mut items_to_pack: Vec<Item> = order_list
//             .iter()
//             .filter(|order| !order.items.is_empty())
//             .flat_map(|order| order.items.clone())
//             .collect();

//         // Sort once, after populating
//         items_to_pack.sort_by(|a, b| b.get_volume().partial_cmp(&a.get_volume()).unwrap());

//         let items_to_pack_count = items_to_pack.len();
//         let mut open_pivots: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0]];

//         for item in items_to_pack {
//             let new_pivots = pack_item_to_bin(
//                 bin,
//                 &item,
//                 &open_pivots,
//                 item.stackable.unwrap(),
//                 item.orientable.unwrap(),
//             );

//             if let Some(new_pivots) = new_pivots {
//                 open_pivots = new_pivots;
//             }
//         }

//         if items_to_pack_count == bin.packed_items.len() {
//             return Some(bin.clone());
//         }
//     }
//     None
// }
