use axum::{
    response::{Html, Json},
    routing::{get, post},
    Router,
};

use serde_json::json;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/clicked", post(clicked));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {
    let mut contents: String = include_str!("index.html").to_string();

    contents += "<script>";
    contents += &include_str!("main.js").to_string();
    contents += "</script>";

    Html(contents)
}

async fn clicked() -> Json<serde_json::Value> {
    println!("Button was clicked!");
    Json(json!({ "status": "ok", "message": "Rust says hello!" }))
}
