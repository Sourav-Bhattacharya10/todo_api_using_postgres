// use axum::{ Router, routing::get, response::Json };
// use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr};
// use tokio_postgres::{Client, NoTls, Row};
// use migration::{Migrator, MigratorTrait};

// use crate::todos::todo_model::TodoModel;

// use serde::{ self, Serialize, Deserialize };

// #[derive(Serialize, Deserialize)]
// struct Person {
//     person_id: i32,
//     person_name: String
// }

// impl From<Row> for Person {
//     fn from(value: Row) -> Self {
//         Self {
//             person_id: value.get(0),
//             person_name: value.get(1)
//         }
//     }
// }

// pub fn todo_routes() -> Router {
//    Router::new()
//     .route("/", get(hello_world_handler))
//     .route("/check", get(check_db))
//     .route("/todos", get(all_todos))
// }

// async fn hello_world_handler() -> &'static str {
//     "Hello to Axum World"
// }

// async fn check_db() -> &'static str {
//     let database_url = "postgres://postgres:postgres@localhost:5432/postgres";
//     let connection = sea_orm::Database::connect(database_url).await.unwrap();
//     let db_backend = connection.get_database_backend();
//     db_backend.
//     let _ = Migrator::up(&connection, None).await;

//     "Hello Check"
// }

// async fn all_todos() -> Json<Vec<TodoModel>> {
//     // let client = connect_postgres().await.unwrap();
//     // let rows = client.query("SELECT  person_id, person_name FROM public.person LIMIT 10", &[]).await.unwrap();
//     // let persons = convert_to_struct(rows);
//     let todo1 = TodoModel { task_id: String::from("a1"), task_name: String::from("Buy Apple"), done_status: false};
//     let todo2 = TodoModel { task_id: String::from("a2"), task_name: String::from("Buy Banana"), done_status: true};
//     let todos = vec![todo1, todo2];
//     Json(todos)
// }

// fn convert_to_struct(rows: Vec<Row>) -> Vec<Person> {
//     let mut persons: Vec<Person> = Vec::new();
//     for row in rows {
//         let person = Person::from(row);
//         persons.push(person)
//     }

//     persons
// }

// async fn connect_postgres() -> Result<(), DbErr> {
//     // let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls).await?;

//     // tokio::spawn(async move {
//     //     if let Err(e) = connection.await {
//     //         eprintln!("Connection Error : {}", e);
//     //     }
//     // });

//     let _db: DatabaseConnection = Database::connect("postgres://postgres:postgres@localhost:5432/postgres").await?;

//     Ok(())
// }

use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::prelude::Todo;
use crate::entities::todo;
use crate::error_responder::ErrorResponder;

#[get("/")]
pub async fn get_all_todos(
    db_conn: &State<DatabaseConnection>,
) -> Result<Json<Vec<todo::Model>>, ErrorResponder> {
    let db = db_conn as &DatabaseConnection;

    let todos = Todo::find()
        .all(db)
        .await
        .map_err(Into::<ErrorResponder>::into)?
        .into_iter()
        .collect();

    Ok(Json(todos))
}
