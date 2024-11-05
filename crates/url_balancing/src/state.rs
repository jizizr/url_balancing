use oauth2::basic::BasicClient;

use crate::{
    dao::{mysql::db::DbSrv, redis::db::RdSrv},
    error::AppError,
};

pub struct AppState {
    pub mdb: DbSrv,
    pub rdb: RdSrv,
    pub oauth2_client: BasicClient,
}

impl AppState {
    pub async fn add_key(&self, uid: i64, key: &str, limitation: i16) -> Result<(), AppError> {
        self.rdb.add_key(uid, key, limitation).await?;
        self.mdb.add_key(uid, key).await?;
        Ok(())
    }

    pub async fn check_key(&self, uid: Option<i64>, key: &str) -> Result<bool, AppError> {
        if self.rdb.check_key(uid, key).await? {
            return Ok(true);
        }
        if self.mdb.check_key(key).await?.is_some() {
            self.rdb.add_key(uid.unwrap(), key, 100).await?;
            return Ok(true);
        }
        Ok(false)
    }

    pub async fn get_url(&self, key: &str) -> Result<Option<String>, AppError> {
        let url = self.rdb.get_url(key).await?;
        if url.is_some() {
            return Ok(url);
        }
        let urls = self.mdb.get_urls(key).await?;
        if urls.is_empty() {
            return Ok(None);
        }
        for url in urls {
            self.rdb.add_url(key, &url).await?;
        }
        Box::pin(self.get_url(key)).await
    }

    pub async fn add_url(&self, key: &str, url: &str) -> Result<(), AppError> {
        self.rdb.add_url(key, url).await?;
        self.mdb.add_url(key, url).await?;
        Ok(())
    }

    pub async fn delete_url(&self, key: &str, url: &str) -> Result<(), AppError> {
        self.mdb.delete_url(key, url).await?;
        self.rdb.delete_url(key, url).await?;
        Ok(())
    }

    pub async fn get_urls(&self, key: &str) -> Result<Vec<String>, AppError> {
        let urls = self.rdb.get_urls(key).await?;
        if urls.is_empty() {
            let urls = self.mdb.get_urls(key).await?;
            for url in &urls {
                self.rdb.add_url(key, url).await?;
            }
            return Ok(urls);
        }
        Ok(urls)
    }

    pub async fn set_csrf(&self, csrf: &str) -> Result<(), AppError> {
        self.rdb.set_csrf(csrf).await
    }

    pub async fn check_csrf(&self, csrf: &str) -> Result<bool, AppError> {
        self.rdb.check_csrf(csrf).await
    }

    pub async fn get_user_keys(&self, uid: i64) -> Result<Vec<String>, AppError> {
        let keys = self.rdb.get_user_keys(uid).await?;
        if !keys.is_empty() {
            return Ok(keys);
        }
        let keys = self.mdb.get_user_keys(uid).await?;
        let keys_str: Vec<String> = keys.into_iter().map(|key| key.key).collect();
        for key in &keys_str {
            self.rdb.add_key(uid, key, 100).await?;
        }
        Ok(keys_str)
    }
}
