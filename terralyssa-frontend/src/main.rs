use axum::{
    response::Json,
    routing::post,
    Router,
};

use serde_json::json;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let static_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("static");
    let static_files = ServeDir::new(static_path).append_index_html_on_directories(true);

    // build our application with a route
    let app = Router::new()
        .route("/clicked", post(clicked))
        .nest_service("/", static_files);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn clicked() -> Json<serde_json::Value> {
    println!("Button was clicked!");
    Json(json!({ "status": "ok", "message": "Rust says hello!" }))
}
