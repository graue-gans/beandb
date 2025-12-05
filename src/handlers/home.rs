use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "about.html")] // Change to about.html to avoid confusion
struct AboutTemplate {
    title: String,
    site_name: String,
    tagline: String,
    footer_text: String,
}

pub async fn about_page() -> Response {
    let template = AboutTemplate {
        title: "bean db - about".to_string(),
        site_name: "Coffee Bean Tracker".to_string(),
        tagline: "Track your specialty coffee journey".to_string(),
        footer_text: "» by max kehrer".to_string(),
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response()
        }
    }
}
