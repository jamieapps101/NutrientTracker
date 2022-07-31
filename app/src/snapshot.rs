use chrono::Utc;
use sea_orm::{query::Statement, ConnectionTrait, DatabaseConnection};
use std::path::PathBuf;

async fn create_snapshot(
    conn: &DatabaseConnection,
    db_name: &str,
    backup_dir: Option<PathBuf>,
) -> Result<(), ()> {
    let backend = conn.get_database_backend();
    let date_string = Utc::now().format("%H_%M_%S__%d_%m_%Y").to_string();
    let file_name = PathBuf::from(date_string + ".sql");
    let file_path = if let Some(mut dir) = backup_dir {
        dir.push(file_name);
        dir
    } else {
        file_name
    };
    let stmt = Statement::from_string(
        backend,
        format!("pg_dump {db_name} > {}", file_path.to_str().unwrap()),
    );
    let res = conn.execute(stmt).await;

    Ok(())
}
