use crate::bin_packing::item::{Item, RawItem};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawOrder {
    pub name: String,
    pub items: Vec<RawItem>,
    pub orientable: Option<bool>,
    pub stackable: Option<bool>,
}

impl RawOrder {
    pub fn create_order_from_raw_order(&self) -> Order {
        let mut items: Vec<Item> = Vec::new();
        for item in &self.items {
            // for each raw item, create an item with quantity 1
            for i in 0..item.quantity {
                items.push(Item {
                    name: self.name.clone()
                        + "@"
                        + item.name.clone().as_str()
                        + "@"
                        + i.to_string().as_str(),
                    width: item.width,
                    depth: item.depth,
                    height: item.height,
                    weight: item.weight,
                    rotation: String::from("RT_WHD"),
                    position: [0.0, 0.0, 0.0],
                    orientable: self.orientable,
                    stackable: self.stackable,
                });
            }
        }
        return Order {
            name: self.name.clone(),
            items,
        };
    }
}

#[derive(Clone, Debug)]
pub struct Order {
    pub name: String,
    pub items: Vec<Item>,
}

impl Order {
    pub fn get_order_volume(&self) -> f32 {
        let mut total_volume = 0.0;
        for item in &self.items {
            total_volume += item.get_volume();
        }
        return total_volume;
    }

    pub fn get_order_weight(&self) -> f32 {
        let mut total_weight = 0.0;
        for item in &self.items {
            total_weight += item.weight;
        }
        return total_weight;
    }
}

// DimensionLessOrder has items with no given dimensions, hence only weight and/or volume is considered

#[derive(Serialize, Deserialize, Clone)]
pub struct DimensionLessOrder {
    pub id: String,
    pub weight: f64,
    pub volume: Option<f64>,
}
