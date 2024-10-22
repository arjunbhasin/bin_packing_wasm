use crate::bin_packing::order::DimensionLessOrder;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

// Knapsack function for bins without volume constraints
pub fn knapsack_1d_float(items: &[DimensionLessOrder], capacity: f64) -> Vec<DimensionLessOrder> {
    let mut dp: BTreeMap<OrderedFloat<f64>, (f64, Vec<DimensionLessOrder>)> = BTreeMap::new();
    dp.insert(OrderedFloat(0.0), (0.0, Vec::new()));

    for item in items {
        let mut new_dp = dp.clone();
        for (&w_ordered, &(val, ref packed_items)) in dp.iter() {
            let w = w_ordered.into_inner();
            let new_w = w + item.weight;
            if new_w <= capacity {
                let new_val = val + item.weight;
                let new_w_ordered = OrderedFloat(new_w);
                match new_dp.get(&new_w_ordered) {
                    Some(&(existing_val, _)) if existing_val >= new_val => (),
                    _ => {
                        let mut new_packed_items = packed_items.clone();
                        new_packed_items.push(item.clone());
                        new_dp.insert(new_w_ordered, (new_val, new_packed_items));
                    }
                }
            }
        }
        dp = new_dp;
    }

    // Find the best value
    let best_packed_items = dp
        .iter()
        .max_by(|a, b| a.1 .0.partial_cmp(&b.1 .0).unwrap())
        .map(|(_, v)| v.1.clone())
        .unwrap();

    best_packed_items
}

// Knapsack function for bins with volume constraints
pub fn knapsack_2d_float(
    items: &[DimensionLessOrder],
    capacity_weight: f64,
    capacity_volume: f64,
) -> Vec<DimensionLessOrder> {
    let mut dp: BTreeMap<(OrderedFloat<f64>, OrderedFloat<f64>), (f64, Vec<DimensionLessOrder>)> =
        BTreeMap::new();
    dp.insert((OrderedFloat(0.0), OrderedFloat(0.0)), (0.0, Vec::new()));

    for item in items {
        let item_weight = item.weight;
        let item_volume = item.volume.unwrap_or(0.0);

        let mut new_dp = dp.clone();
        for (&(w_ordered, v_ordered), &(val, ref packed_items)) in dp.iter() {
            let w = w_ordered.into_inner();
            let v = v_ordered.into_inner();

            let new_w = w + item_weight;
            let new_v = v + item_volume;

            if new_w <= capacity_weight && new_v <= capacity_volume {
                let new_val = val + item_weight;
                let key = (OrderedFloat(new_w), OrderedFloat(new_v));

                match new_dp.get(&key) {
                    Some(&(existing_val, _)) if existing_val >= new_val => (),
                    _ => {
                        let mut new_packed_items = packed_items.clone();
                        new_packed_items.push(item.clone());
                        new_dp.insert(key, (new_val, new_packed_items));
                    }
                }
            }
        }
        dp = new_dp;
    }

    // Find the best value
    let best_packed_items = dp
        .iter()
        .max_by(|a, b| a.1 .0.partial_cmp(&b.1 .0).unwrap())
        .map(|(_, v)| v.1.clone())
        .unwrap();

    best_packed_items
}
