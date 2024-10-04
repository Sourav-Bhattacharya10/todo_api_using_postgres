use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sea_orm::{sea_query::extension::postgres::PgExpr, DbErr};
use sea_orm::{sea_query::Expr, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, ModelTrait, QueryOrder, QueryTrait};

use crate::entities::todo;
use crate::params::Params;
use crate::{entities::prelude::Todo, error_responder::ErrorResponder};

use super::dtos::todo_task::TodoTask;
use super::dtos::{paginated_todo::PaginatedTodo, todo_dto::TodoDto};

pub fn get_todo_router() -> Router<DatabaseConnection> {
    let todo_routes = Router::new()
        .route("/", get(get_all_todos).post(create_new_todo))
        .route(
            "/:id",
            get(get_todo_by_id)
                .put(edit_todo_task_name)
                .patch(toggle_todo_done_status)
                .delete(delete_existing_todo),
        );

    let todo_router = Router::new().nest("/todos", todo_routes);

    todo_router
}

pub async fn get_all_todos(
    State(db_conn): State<DatabaseConnection>,
    Query(params): Query<Params>,
) -> Result<Json<PaginatedTodo>, ErrorResponder> {
    let db = db_conn as DatabaseConnection;

    let page = params.page;
    let page_size = params.page_size;
    let term = params.term.unwrap_or(String::from(""));
    let done_status = params.done_status;

    let search_term_filter = format!("%{}%", term);

    let todo_pages = Todo::find()
        .apply_if(done_status, |query, v| {
            query.filter(todo::Column::DoneStatus.eq(v))
        })
        .filter(Expr::col(todo::Column::TaskName).ilike(search_term_filter))
        .order_by_asc(todo::Column::TaskId)
        .paginate(&db, page_size);

    let todos = todo_pages.fetch_page(page - 1).await?;
    let todos_nums_pages = todo_pages.num_items_and_pages().await?;

    let todo_dtos = todos.into_iter().map(|todo| todo.into()).collect();

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

pub async fn get_todo_by_id(
    State(db_conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<TodoDto>, ErrorResponder> {
    let db = db_conn as DatabaseConnection;

    let todo = Todo::find_by_id(id).one(&db).await?;

    Ok(if let Some(todo) = todo {
        let todo_dto: TodoDto = todo.into();
        Json(todo_dto)
    } else {
        return Err(ErrorResponder::DatabaseError(DbErr::RecordNotFound(
            format!("No todo found with id {id}").into(),
        )));
    })
}

pub async fn create_new_todo(
    State(db_conn): State<DatabaseConnection>,
    Json(todo_task): Json<TodoTask>,
) -> Result<Json<TodoDto>, ErrorResponder> {
    let db = db_conn.clone() as DatabaseConnection;

    let new_todo = todo::ActiveModel {
        task_name: ActiveValue::Set(todo_task.task_name.clone()),
        done_status: ActiveValue::Set(false),
        ..Default::default()
    };

    let response = Todo::insert(new_todo).exec(&db).await?;

    let added_todo = get_todo_by_id(State(db_conn), Path(response.last_insert_id)).await?;

    Ok(added_todo)
}

pub async fn edit_todo_task_name(
    State(db_conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(todo_task): Json<TodoTask>,
) -> Result<Json<TodoDto>, ErrorResponder> {
    let db = db_conn.clone() as DatabaseConnection;

    let existing_todo = get_todo_by_id(State(db_conn.clone()), Path(id)).await?;

    let updated_todo = todo::ActiveModel {
        task_id: ActiveValue::Set(id),
        task_name: ActiveValue::Set(todo_task.task_name.clone()),
        done_status: ActiveValue::Set(existing_todo.done_status),
    };

    updated_todo.update(&db).await?;

    let existing_todo = get_todo_by_id(State(db_conn.clone()), Path(id)).await?;

    Ok(existing_todo)
}

pub async fn toggle_todo_done_status(
    State(db_conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<TodoDto>, ErrorResponder> {
    let db = db_conn.clone() as DatabaseConnection;

    let existing_todo = get_todo_by_id(State(db_conn.clone()), Path(id)).await?;

    let updated_todo = todo::ActiveModel {
        task_id: ActiveValue::Set(existing_todo.task_id),
        task_name: ActiveValue::Set(existing_todo.task_name.clone()),
        done_status: ActiveValue::Set(!existing_todo.done_status),
    };

    updated_todo.update(&db).await?;

    let existing_todo = get_todo_by_id(State(db_conn.clone()), Path(id)).await?;

    Ok(existing_todo)
}

pub async fn delete_existing_todo(
    State(db_conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ErrorResponder> {
    let db = db_conn.clone() as DatabaseConnection;

    let todo = Todo::find_by_id(id).one(&db).await?;

    if let Some(todo) = todo {
        todo.delete(&db).await?;
    }

    Ok(StatusCode::NO_CONTENT)
}
