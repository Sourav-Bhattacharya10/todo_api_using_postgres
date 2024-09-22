mod cors;
mod entities;
mod error_responder;
mod setup;
mod todos;

use rocket::*;

use crate::cors::Cors;
use crate::setup::setup_db;
use todos::todo_routes::{
    create_new_todo, delete_existing_todo, edit_todo_task_name, get_all_todos, get_todo_by_id,
    toggle_todo_done_status,
};

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
        .mount(
            "/todos",
            routes![
                create_new_todo,
                delete_existing_todo,
                edit_todo_task_name,
                get_all_todos,
                get_todo_by_id,
                toggle_todo_done_status,
            ],
        )
}

#[get("/health")]
async fn health() -> &'static str {
    "OK Hello World"
}
