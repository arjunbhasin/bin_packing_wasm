mod bin_packing;
use bin_packing::bin::{Bin, DLBinWithPackedOrders, RawBin};
use bin_packing::item::Item;
use bin_packing::order::{DimensionLessOrder, Order, RawOrder};
use bin_packing::solver::get_smallest_fitting_bin_for_item_vector;
use bin_packing::solver::knapsack::{knapsack_1d_float, knapsack_2d_float};
use bin_packing::sort_bin_list_by_weight;
use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_wasm() -> String {
    "WASM Loaded...".to_string()
}

#[wasm_bindgen]
pub fn get_smallest_fitting_bin_for_order_list(
    js_orders: JsValue,
    js_bins: JsValue,
    js_update_function: &Function,
) -> Result<JsValue, JsValue> {
    // raw orders are orders from frontend
    let raw_orders: Vec<RawOrder> = serde_wasm_bindgen::from_value(js_orders)?;
    // raw bins are bins from frontend
    let raw_bins: Vec<RawBin> = serde_wasm_bindgen::from_value(js_bins)?;

    // make orders from raw_orders
    let orders: Vec<Order> = raw_orders
        .iter()
        .map(|order| order.create_order_from_raw_order())
        .collect();

    // make bin list from raw_bins
    let mut bins: Vec<Bin> = raw_bins.iter().map(|bin| bin.convert_to_bin()).collect();

    let total_order_weight: f32 = orders.iter().map(|order| order.get_order_weight()).sum();
    let total_order_volume: f32 = orders.iter().map(|order| order.get_order_volume()).sum();

    // flatten all items from all orders into a single list
    let mut item_list = orders
        .iter()
        .flat_map(|order| order.items.clone())
        .collect::<Vec<Item>>();

    // sort item list by weight
    item_list.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());
    sort_bin_list_by_weight(&mut bins, true);

    match get_smallest_fitting_bin_for_item_vector(
        &item_list,
        &mut bins,
        total_order_weight,
        total_order_volume,
        js_update_function,
    ) {
        Some(bin) => Ok(serde_wasm_bindgen::to_value(&bin)?),
        None => Ok(serde_wasm_bindgen::to_value(&0)?),
    }
}

/*
    This function is used to pack max additional DIMENSION-LESS orders into DIMENSION-LESS bins that already have packed orders
*/

#[wasm_bindgen]
pub fn pack_max_dimensionless_orders(
    dl_bins_with_packed_orders: JsValue,
    additional_orders: JsValue, // must be dimensionless orders
) -> Result<JsValue, JsValue> {
    // these bins may have packed orders
    let mut dl_bins_with_packed_orders: Vec<DLBinWithPackedOrders> =
        serde_wasm_bindgen::from_value(dl_bins_with_packed_orders)?;
    let additional_orders: Vec<DimensionLessOrder> =
        serde_wasm_bindgen::from_value(additional_orders)?;

    // For each bin, select additional orders to pack
    for bin in dl_bins_with_packed_orders.iter_mut() {
        // Calculate remaining capacities
        let used_weight: f64 = bin.packed_orders.iter().map(|o| o.weight).sum();
        let remaining_weight = bin.max_weight - used_weight;

        let remaining_volume = if let Some(max_vol) = bin.max_volume {
            let used_volume: f64 = bin
                .packed_orders
                .iter()
                .map(|o| o.volume.unwrap_or(0.0))
                .sum();
            Some(max_vol - used_volume)
        } else {
            None
        };

        // Filter orders that can potentially fit by weight and volume constraints
        let candidate_orders: Vec<DimensionLessOrder> = additional_orders
            .iter()
            .filter(|o| o.weight <= remaining_weight)
            .cloned()
            .collect();

        let selected_orders = if let Some(remaining_vol) = remaining_volume {
            // Bin has volume constraint, use 2D Knapsack
            let capacity_weight = remaining_weight;
            let capacity_volume = remaining_vol;
            let orders_with_volume: Vec<DimensionLessOrder> = candidate_orders
                .into_iter()
                .filter(|o| o.volume.is_some() && o.volume.unwrap() <= capacity_volume)
                .collect();
            knapsack_2d_float(&orders_with_volume, capacity_weight, capacity_volume)
        } else {
            // Bin has no volume constraint, use 1D Knapsack
            knapsack_1d_float(&candidate_orders, remaining_weight)
        };

        // Assign the selected orders to the bin
        bin.additional_packed_orders = selected_orders;
    }
    // Remove bins that have no additional orders to pack
    dl_bins_with_packed_orders.retain(|bin| !bin.additional_packed_orders.is_empty());

    Ok(serde_wasm_bindgen::to_value(&dl_bins_with_packed_orders)?)
}
