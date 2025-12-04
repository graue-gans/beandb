use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Beans {
    pub id: Option<i32>,
    pub name: String,
    pub flavor_profile: String,
    pub country: String,
    pub region: String,
    pub variety: String,
    pub process: String,
    pub roaster: String,
    pub roast_level: String, // Changed from enum to String for form compatibility
    pub purchase_date: String,
    pub price: f32,
    pub weight: i32,
    pub bought_at: String,
    pub rating: i32, // 1 - 10
    pub notes: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrewingMethod {
    pub id: Option<i32>,
    pub bean_id: i32,
    pub method: String,
    pub grind_size: String,
    pub brewing_ratio: String,
    pub rating: i32,
    pub notes: String,
}

// Keep the enum if you want to use it later for validation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RoastLevel {
    VeryLight,
    Light,
    Medium,
    Dark,
}
