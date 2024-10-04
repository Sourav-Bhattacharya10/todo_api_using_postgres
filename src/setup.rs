use dotenv::dotenv;
use sea_orm::*;

pub async fn setup_db() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();

    let databse_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in environment variables");
    let db_name: String = std::env::var("DB_NAME").expect("DB_NAME not found in environment variables");

    let url = format!("{}/{}", databse_url, db_name);
    let db_conn = Database::connect(&url).await?;

    Ok(db_conn)
}
