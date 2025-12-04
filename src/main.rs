mod models;
// use models::beans::Beans;

mod handlers;

use axum::{routing::get, Router};
use handlers::beans::{add_beans, view_beans};
use handlers::home::landing_page;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(landing_page))
        .route("/beans", get(view_beans))
        .route("/beans/new", get(add_beans))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Coffee Tracker running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
