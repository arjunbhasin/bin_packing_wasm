use crate::bin_packing::item::Item;
use serde::{Deserialize, Serialize};

use super::order::DimensionLessOrder;

#[derive(Serialize, Clone, Debug)]
pub struct Bin {
    pub name: String,
    #[serde(skip_serializing)]
    pub width: f32,
    #[serde(skip_serializing)]
    pub depth: f32,
    #[serde(skip_serializing)]
    pub height: f32,
    #[serde(skip_serializing)]
    pub max_weight: f32,
    pub packed_items: Vec<Item>,
}

impl Bin {
    pub fn get_volume(&self) -> f32 {
        self.width * self.depth * self.height
    }

    pub fn get_packed_items_volume(&self) -> f32 {
        let mut volume: f32 = 0.0;
        for item in &self.packed_items {
            volume += &item.get_volume();
        }
        volume
    }

    pub fn get_packed_items_weight(&self) -> f32 {
        let mut weight: f32 = 0.0;
        for item in &self.packed_items {
            weight += &item.weight;
        }
        weight
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawBin {
    pub name: String,
    pub depth: f32,
    pub width: f32,
    pub height: f32,
    pub max_weight: f32,
}

impl RawBin {
    pub fn convert_to_bin(&self) -> Bin {
        Bin {
            name: self.name.clone(),
            width: self.width,
            depth: self.depth,
            height: self.height,
            max_weight: self.max_weight,
            packed_items: Vec::new(),
        }
    }
}

// used only for DimensionLessOrder
#[derive(Serialize, Deserialize)]
pub struct BinWithPackedOrders {
    pub id: String,
    pub max_weight: f64,
    pub max_volume: Option<f64>,
    pub packed_orders: Vec<DimensionLessOrder>,
    // Field to store the result
    pub additional_packed_orders: Vec<DimensionLessOrder>,
}
