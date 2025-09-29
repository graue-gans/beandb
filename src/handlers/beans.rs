use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "beans_overview.html")]
struct BeansOverviewTemplate {
    title: String,
    site_name: String,
    tagline: String,
    bean_entries: Vec<BeanEntry>,
    footer_text: String,
}

struct BeanEntry {
    icon: String,
    title: String,
    description: String,
}

pub async fn view_beans() -> Response {
    let bean_entries = vec![
        BeanEntry {
            icon: "A".to_string(),
            title: "Kianderi Ab".to_string(),
            description: "Singe Estate coffee".to_string(),
        },
        BeanEntry {
            icon: "B".to_string(),
            title: "Oonikat Kenia".to_string(),
            description: "SL28".to_string(),
        },
        BeanEntry {
            icon: "C".to_string(),
            title: "Canephora Suedhang".to_string(),
            description: "Yeast-fermented processing".to_string(),
        },
    ];

    let template = BeansOverviewTemplate {
        title: "bean db".to_string(),
        site_name: "BEANDB".to_string(),
        tagline: "View all registered beans".to_string(),
        bean_entries,
        footer_text: "» BeanDB by Max Kehrer".to_string(),
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response(),
    }
}

// pub async fn add_bean() -> Response {
//     let template = BeansOverviewTemplate {
//         title: "bean db".to_string(),
//         site_name: "BEANDB".to_string(),
//         tagline: "Track your coffee journey, one bean at a time".to_string(),
//         footer_text: "» BeanDB by Max Kehrer".to_string(),
//     };

//     match template.render() {
//         Ok(html) => Html(html).into_response(),
//         Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response(),
//     }
// }
