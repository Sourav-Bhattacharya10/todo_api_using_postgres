mod entities;
mod error_responder;
mod setup;
mod todos;
mod cors;

use rocket::*;

use crate::setup::setup_db;
use todos::todo_routes::get_all_todos;
use crate::cors::Cors;

#[launch] // The "main" function of the program
async fn rocket() -> _ {
    let db_conn = match setup_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db_conn)
        .attach(Cors)
        .mount("/", routes![health])
        .mount("/todos", routes![get_all_todos])
}

#[get("/health")]
async fn health() -> &'static str {
    "OK Hello World"
}
