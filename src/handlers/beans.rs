use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use serde::Deserialize;
use std::sync::Arc;

use crate::database::Database;
use crate::models::beans::{Beans, BrewingMethod};

#[derive(Template)]
#[template(path = "beans_overview.html")]
struct BeansOverviewTemplate {
    title: String,
    site_name: String,
    tagline: String,
    bean_entries: Vec<BeanEntry>,
    footer_text: String,
}

#[derive(Template)]
#[template(path = "beans_add.html")]
struct BeansAddTemplate {
    title: String,
    site_name: String,
    tagline: String,
    footer_text: String,
}

struct BeanEntry {
    name: String,
    origin: String,
    roaster: String,
    variety: String,
    process: String,
    rating: i32,
}

#[derive(Deserialize)]
pub struct BeanFormData {
    name: String,
    flavor_profile: String,
    country: String,
    region: String,
    variety: String,
    process: String,
    roaster: String,
    roast_level: String,
    purchase_date: String,
    price: f32,
    weight: i32,
    bought_at: String,
    brewing_method: String,
    grind_size: String,
    brewing_ratio: String,
    brewing_rating: i32,
    brewing_notes: String,
    rating: i32,
    notes: String,
}

pub async fn view_beans(State(db): State<Arc<Database>>) -> Response {
    let beans = match db.get_all_beans() {
        Ok(beans) => beans,
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let bean_entries: Vec<BeanEntry> = beans
        .iter()
        .map(|bean| {
            // Format origin as "Country, Region" or just "Country" if region is empty
            let origin = if bean.region.is_empty() {
                bean.country.clone()
            } else {
                format!("{}, {}", bean.country, bean.region)
            };

            BeanEntry {
                name: bean.name.clone(),
                origin,
                roaster: bean.roaster.clone(),
                variety: bean.variety.clone(),
                process: bean.process.clone(),
                rating: bean.rating,
            }
        })
        .collect();

    let template = BeansOverviewTemplate {
        title: "bean db - overview".to_string(),
        site_name: "BEANDB".to_string(),
        tagline: "View all registered beans".to_string(),
        bean_entries,
        footer_text: "» about beandb".to_string(),
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response()
        }
    }
}

pub async fn add_beans() -> Response {
    let template = BeansAddTemplate {
        title: "bean db - add".to_string(),
        site_name: "BEANDB".to_string(),
        tagline: "Add new bean to database".to_string(),
        footer_text: "» about beandb".to_string(),
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response()
        }
    }
}

pub async fn create_bean(
    State(db): State<Arc<Database>>,
    Form(form_data): Form<BeanFormData>,
) -> Response {
    let bean = Beans {
        id: None,
        name: form_data.name,
        flavor_profile: form_data.flavor_profile,
        country: form_data.country,
        region: form_data.region,
        variety: form_data.variety,
        process: form_data.process,
        roaster: form_data.roaster,
        roast_level: form_data.roast_level,
        purchase_date: form_data.purchase_date,
        price: form_data.price,
        weight: form_data.weight,
        bought_at: form_data.bought_at,
        rating: form_data.rating,
        notes: form_data.notes,
    };

    let brewing_method = BrewingMethod {
        id: None,
        bean_id: 0, // Will be set by insert_bean
        method: form_data.brewing_method,
        grind_size: form_data.grind_size,
        brewing_ratio: form_data.brewing_ratio,
        rating: form_data.brewing_rating,
        notes: form_data.brewing_notes,
    };

    match db.insert_bean(&bean, &brewing_method) {
        Ok(_) => Redirect::to("/").into_response(),
        Err(e) => {
            eprintln!("Failed to add bean: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to add bean").into_response()
        }
    }
}
