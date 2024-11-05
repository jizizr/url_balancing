use sea_orm::Database;
use std::env;

use super::*;

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let db_url = env::var("DATABASE_URL").unwrap();
    let db = Database::connect(db_url).await.expect("连接数据库失败");
    Migrator::up(&db, None).await.expect("迁移失败");
    Ok(db)
}
