use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{sea_query::Expr, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use sea_orm::{ColumnTrait, QueryOrder, QueryTrait};

use crate::entities::paginated_todo::PaginatedTodo;
use crate::entities::prelude::Todo;
use crate::entities::todo;
use crate::error_responder::ErrorResponder;

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

    let paged_todo = PaginatedTodo {
        todos,
        page,
        page_size,
        total_records: todos_nums_pages.number_of_items,
        order: String::from("asc"),
        total_pages: todos_nums_pages.number_of_pages,
    };

    Ok(Json(paged_todo))
}
