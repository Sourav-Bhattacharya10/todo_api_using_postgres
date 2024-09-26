use rocket::Responder;
use sea_orm::DbErr;

#[derive(Responder, Debug)]
#[response(status = 500, content_type = "json")]
pub struct ErrorResponder {
    message: String,
}

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> Self {
        Self { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> Self {
        Self {
            message: str.to_owned().into(),
        }
    }
}
