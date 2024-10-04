mod entities;
mod error_responder;
mod params;
mod setup;
mod todos;

use axum::{
    http::{Method, StatusCode},
    routing::get,
    Router,
};
use sea_orm::{DbErr, RuntimeErr};
use tower_http::cors::{Any, CorsLayer};

use error_responder::ErrorResponder;
use setup::setup_db;
use todos::todo_routes::get_todo_router;

#[tokio::main] // The "main" function of the program
async fn main() {
    let db_conn = match setup_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    let health_router = Router::new().route("/health", get(health));
    let todo_router = get_todo_router();

    let app = Router::new()
        .merge(health_router)
        .merge(todo_router)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::PATCH])
                .allow_headers(Any),
        )
        .with_state(db_conn);

    let port = "8000";
    println!("Server started running on http://localhost:{port}");
    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Result<(StatusCode, &'static str), ErrorResponder> {
    let db_conn = setup_db().await;

    Ok(if db_conn.is_ok() {
        (
            StatusCode::OK,
            "Application and Database are up and running",
        )
    } else {
        return Err(ErrorResponder::DatabaseError(DbErr::Conn(
            RuntimeErr::Internal(String::from("Unable to connect to database")),
        )));
    })
}
