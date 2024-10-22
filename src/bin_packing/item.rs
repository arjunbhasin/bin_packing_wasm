use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RawItem {
    pub name: String,
    pub quantity: i32,
    pub depth: f32,
    pub width: f32,
    pub height: f32,
    pub weight: f32,
}

/*
    Note: Item has quantity 1 by default
    Modify incoming order list that has quantity > 1 to have quantity 1
*/
#[derive(Serialize, Clone, Debug)]
pub struct Item {
    pub name: String,
    #[serde(skip_serializing)]
    pub width: f32,
    #[serde(skip_serializing)]
    pub depth: f32,
    #[serde(skip_serializing)]
    pub height: f32,
    #[serde(skip_serializing)]
    pub weight: f32,
    // skip serialization for these fields
    #[serde(skip_serializing)]
    pub orientable: Option<bool>,
    #[serde(skip_serializing)]
    pub stackable: Option<bool>,
    pub rotation: String,
    pub position: [f32; 3],
}

impl Item {
    pub fn get_volume(&self) -> f32 {
        self.width * self.depth * self.height
    }

    pub fn get_rotated_dimension(&self, key: &str) -> [f32; 3] {
        match key {
            "RT_WHD" => [self.width, self.height, self.depth],
            "RT_HWD" => [self.height, self.width, self.depth],
            "RT_HDW" => [self.height, self.depth, self.width],
            "RT_DHW" => [self.depth, self.height, self.width],
            "RT_DWH" => [self.depth, self.width, self.height],
            "RT_WDH" => [self.width, self.depth, self.height],
            _ => [self.width, self.height, self.depth], // default
        }
    }
}
