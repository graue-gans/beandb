mod database;
mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use database::Database;
use handlers::beans::{add_beans, create_bean, view_beans};
use handlers::home::about_page; // Renamed from landing_page
use std::sync::Arc;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Initialize database
    let db = Arc::new(Database::new("coffee.db").expect("Failed to open database"));

    db.init_schema()
        .expect("Failed to initialize database schema");

    let app = Router::new()
        .route("/", get(view_beans)) // Changed: beans list is now homepage
        .route("/about", get(about_page)) // Changed: info page at /about
        .route("/add", get(add_beans)) // Changed: /beans/add → /add
        .route("/new", post(create_bean)) // Changed: /beans/new → /new
        .nest_service("/static", ServeDir::new("static"))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Coffee Tracker running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
