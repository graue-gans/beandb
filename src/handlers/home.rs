use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "landing.html")]
struct LandingTemplate {
    title: String,
    site_name: String,
    features: Vec<Feature>,
    footer_text: String,
}

struct Feature {
    icon: String,
    title: String,
    description: String,
}

pub async fn landing_page() -> Response {
    let features = vec![
        Feature {
            icon: "🌍".to_string(),
            title: "Origin".to_string(),
            description: "Country and region of your beans".to_string(),
        },
        Feature {
            icon: "🔥".to_string(),
            title: "Roast Level".to_string(),
            description: "Light, medium, or dark roasts".to_string(),
        },
        Feature {
            icon: "📝".to_string(),
            title: "Notes".to_string(),
            description: "Your tasting notes and ratings".to_string(),
        },
    ];

    let template = LandingTemplate {
        title: "BEANDB".to_string(),
        site_name: "Coffee Bean Tracker".to_string(),
        features,
        footer_text: "» BeanDB by Max Kehrer".to_string(),
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response(),
    }
}
