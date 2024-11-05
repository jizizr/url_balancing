use super::*;
use crate::error::AppError;
use entity::{key, url};
use sea_orm::ActiveValue::Set;

pub struct DbSrv {
    pub db: DbConn,
}

impl DbSrv {
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }
    pub async fn add_key(&self, user_id: i64, key: &str) -> Result<key::Model, AppError> {
        let key = key::ActiveModel {
            user_id: Set(user_id),
            key: Set(key.to_string()),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;
        Ok(key)
    }

    pub async fn check_key(&self, key: &str) -> Result<Option<key::Model>, AppError> {
        let key = key::Entity::find()
            .filter(key::Column::Key.eq(key))
            .one(&self.db)
            .await?;
        Ok(key)
    }

    pub async fn get_user_keys(&self, user_id: i64) -> Result<Vec<key::Model>, AppError> {
        let keys = key::Entity::find()
            .filter(key::Column::UserId.eq(user_id))
            .all(&self.db)
            .await?;
        Ok(keys)
    }

    pub async fn add_url(&self, key: &str, url: &str) -> Result<(), AppError> {
        let key = self.check_key(key).await?;
        if let Some(key) = key {
            url::ActiveModel {
                id: Set(key.id),
                url: Set(url.to_string()),
                ..Default::default()
            }
            .insert(&self.db)
            .await?;
            return Ok(());
        }
        Err(AppError::KeyNotFound)
    }

    pub async fn get_urls(&self, key: &str) -> Result<Vec<String>, AppError> {
        let key = self.check_key(key).await?;
        if let Some(key) = key {
            let urls = url::Entity::find()
                .filter(url::Column::Id.eq(key.id))
                .all(&self.db)
                .await?;
            return Ok(urls.iter().map(|url| url.url.clone()).collect());
        }
        Err(AppError::KeyNotFound)
    }

    pub async fn delete_url(&self, key: &str, url: &str) -> Result<(), AppError> {
        let key = self.check_key(key).await?;
        if let Some(key) = key {
            url::ActiveModel {
                id: Set(key.id),
                url: Set(url.to_string()),
                ..Default::default()
            }
            .delete(&self.db)
            .await?;
            return Ok(());
        }
        Err(AppError::KeyNotFound)
    }

    pub async fn delete_key(&self, key: &str) -> Result<(), AppError> {
        let key = self.check_key(key).await?;
        if let Some(key) = key {
            key.delete(&self.db).await?;
            return Ok(());
        }
        Err(AppError::KeyNotFound)
    }

    pub async fn get_key(&self, key: &str) -> Result<Option<key::Model>, AppError> {
        let key = key::Entity::find()
            .filter(key::Column::Key.eq(key))
            .one(&self.db)
            .await?;
        Ok(key)
    }
}

#[cfg(test)]
use crate::dao::mysql::init;
#[tokio::test]

async fn test_add_key() {
    use std::env;
    let db = init::establish_connection().await.unwrap();
    let db_srv = DbSrv::new(db);
    let key = db_srv.add_key(114514, "test").await.unwrap();
    assert_eq!(key.user_id, 114514);
    assert_eq!(key.key, "test");
    // 输出key
    println!("{:?}", key);
}
