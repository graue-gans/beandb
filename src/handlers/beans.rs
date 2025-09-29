use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "beans_overview.html")]
struct BeansOverviewTemplate {
    title: String,
    site_name: String,
    tagline: String,
    hero_title: String,
    hero_description: String,
    footer_text: String,
}

pub async fn view_beans() -> Response {
    let template = BeansOverviewTemplate {
        title: "bean db".to_string(),
        site_name: "BEANDB".to_string(),
        tagline: "Track your coffee journey, one bean at a time".to_string(),
        hero_title: "Welcome to Your Coffee Collection".to_string(),
        hero_description: "Keep track of all your favorite coffee beans, their origins, roast levels, and tasting notes.".to_string(),
        footer_text: "Happy brewing!".to_string(),
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response(),
    }
}

pub async fn add_bean() -> Response {
    let template = BeansOverviewTemplate {
        title: "bean db".to_string(),
        site_name: "BEANDB".to_string(),
        tagline: "Track your coffee journey, one bean at a time".to_string(),
        hero_title: "Welcome to Your Coffee Collection".to_string(),
        hero_description: "Keep track of all your favorite coffee beans, their origins, roast levels, and tasting notes.".to_string(),
        footer_text: "Happy brewing!".to_string(),
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response(),
    }
}
