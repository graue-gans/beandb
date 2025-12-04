use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Beans {
    id: i32,
    name: String,
    flavor_profile: String,
    country: String,
    region: String,
    variety: String,
    process: String,
    roaster: String,
    roast_level: RoastLevel,
    purchase_date: String,
    price: f32,
    weight: i32,
    bought_at: String,

    brewing_methods: Option<Vec<BrewingMethod>>, // this should be a vec at one point
    rating: i32,                                 // 1 - 10
    notes: String,
    // last_edit: String,
}

#[derive(Serialize, Deserialize)]
enum RoastLevel {
    VeryLight,
    Light,
    Medium,
    Dark,
}

#[derive(Serialize, Deserialize)]
struct BrewingMethod {
    method: String,
    grind_size: String,
    brewing_ratio: String,
    rating: i32,
    notes: String,
}
