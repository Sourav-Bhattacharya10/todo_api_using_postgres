use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;

// Define a custom error type
#[derive(Debug)]
pub enum ErrorResponder {
    DatabaseError(DbErr),
}

// Implement IntoResponse for your custom error type
impl IntoResponse for ErrorResponder {
    fn into_response(self) -> Response {
        match self {
            ErrorResponder::DatabaseError(err) => {
                // Map all other DbErr types to 500 Internal Server Error
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Internal server error {err}"),
                )
                    .into_response()
            }
        }
    }
}

impl From<DbErr> for ErrorResponder {
    fn from(value: DbErr) -> Self {
        Self::DatabaseError(value)
    }
}
