use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, put, State};
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{sea_query::Expr, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, ModelTrait, QueryOrder, QueryTrait};

use crate::entities::prelude::Todo;
use crate::entities::todo;
use crate::error_responder::ErrorResponder;

use super::dtos::paginated_todo::PaginatedTodo;
use super::dtos::todo_task::TodoTask;

#[get("/?<term>&<done_status>&<page>&<page_size>")]
pub async fn get_all_todos(
    db_conn: &State<DatabaseConnection>,
    page: u64,
    page_size: u64,
    term: Option<&str>,
    done_status: Option<bool>,
) -> Result<Json<PaginatedTodo>, ErrorResponder> {
    let db = db_conn as &DatabaseConnection;

    let search_term_filter = format!("%{}%", term.unwrap_or(""));

    let todo_pages = Todo::find()
        .apply_if(done_status, |query, v| {
            query.filter(todo::Column::DoneStatus.eq(v))
        })
        .filter(Expr::col(todo::Column::TaskName).ilike(search_term_filter))
        .order_by_asc(todo::Column::TaskId)
        .paginate(db, page_size);

    let todos = todo_pages.fetch_page(page - 1).await?;
    let todos_nums_pages = todo_pages.num_items_and_pages().await?;

    let todo_dtos = todos.iter().map(|todo| todo.into()).collect();

    let paged_todo = PaginatedTodo {
        todos: todo_dtos,
        page,
        page_size,
        total_records: todos_nums_pages.number_of_items,
        order: String::from("asc"),
        total_pages: todos_nums_pages.number_of_pages,
    };

    Ok(Json(paged_todo))
}

#[get("/<id>")]
pub async fn get_todo_by_id(
    db_conn: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<todo::Model>, ErrorResponder> {
    let db = db_conn as &DatabaseConnection;

    let todo = Todo::find_by_id(id).one(db).await?;

    Ok(if let Some(todo) = todo {
        Json(todo)
    } else {
        return Err(format!("No todo found with id {id}").into());
    })
}

#[post("/", format = "json", data = "<todo_task>")]
pub async fn create_new_todo(
    db_conn: &State<DatabaseConnection>,
    todo_task: Json<TodoTask>,
) -> Result<(Status, Json<todo::Model>), ErrorResponder> {
    let db = db_conn as &DatabaseConnection;

    let new_todo = todo::ActiveModel {
        task_name: ActiveValue::Set(todo_task.task_name.clone()),
        done_status: ActiveValue::Set(false),
        ..Default::default()
    };

    let response = Todo::insert(new_todo).exec(db).await?;

    let added_todo = get_todo_by_id(db_conn, response.last_insert_id).await?;

    Ok((Status::Created, added_todo))
}

#[put("/<id>", format = "json", data = "<todo_task>")]
pub async fn edit_todo_task_name(
    db_conn: &State<DatabaseConnection>,
    id: i32,
    todo_task: Json<TodoTask>,
) -> Result<Json<todo::Model>, ErrorResponder> {
    let db = db_conn as &DatabaseConnection;

    let existing_todo = get_todo_by_id(db_conn, id).await?;

    let updated_todo = todo::ActiveModel {
        task_id: ActiveValue::Set(existing_todo.task_id),
        task_name: ActiveValue::Set(todo_task.task_name.clone()),
        done_status: ActiveValue::Set(existing_todo.done_status),
    };

    updated_todo.update(db).await?;

    let existing_todo = get_todo_by_id(db_conn, id).await?;

    Ok(existing_todo)
}

#[patch("/<id>")]
pub async fn toggle_todo_done_status(
    db_conn: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<todo::Model>, ErrorResponder> {
    let db = db_conn as &DatabaseConnection;

    let existing_todo = get_todo_by_id(db_conn, id).await?;

    let updated_todo = todo::ActiveModel {
        task_id: ActiveValue::Set(existing_todo.task_id),
        task_name: ActiveValue::Set(existing_todo.task_name.clone()),
        done_status: ActiveValue::Set(!existing_todo.done_status),
    };

    updated_todo.update(db).await?;

    let existing_todo = get_todo_by_id(db_conn, id).await?;

    Ok(existing_todo)
}

#[delete("/<id>")]
pub async fn delete_existing_todo(
    db_conn: &State<DatabaseConnection>,
    id: i32,
) -> Result<Status, ErrorResponder> {
    let db = db_conn as &DatabaseConnection;

    let existing_todo = get_todo_by_id(db_conn, id).await?;

    existing_todo.0.delete(db).await?;

    Ok(Status::NoContent)
}
