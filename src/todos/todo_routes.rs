use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{sea_query::Expr, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use sea_orm::{ColumnTrait, QueryTrait};

use crate::entities::prelude::Todo;
use crate::entities::todo;
use crate::error_responder::ErrorResponder;

#[get("/?<term>&<done_status>")]
pub async fn get_all_todos(
    db_conn: &State<DatabaseConnection>,
    term: Option<&str>,
    done_status: Option<bool>,
) -> Result<Json<Vec<todo::Model>>, ErrorResponder> {
    let db = db_conn as &DatabaseConnection;

    let search_term_filter = format!("%{}%", term.unwrap_or(""));

    let todo_pages = Todo::find()
        .apply_if(done_status, |query, v| {
            query.filter(todo::Column::DoneStatus.eq(v))
        })
        .filter(Expr::col(todo::Column::TaskName).ilike(search_term_filter))
        .paginate(db, 10);

    let todos = todo_pages.fetch_page(0).await?;

    Ok(Json(todos))
}
