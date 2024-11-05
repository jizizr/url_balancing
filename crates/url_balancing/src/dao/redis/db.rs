use std::time::{SystemTime, UNIX_EPOCH};

use super::*;

pub struct RdSrv {
    pub db: redis::Client,
}

const REDIS_PREFIX: &str = "URL_BALANCING";
const REDIS_KEY: &str = "KEY";
const REDIS_LIST_PREFIX: &str = "LIST";
const REDIS_CSRF: &str = "CSRF";

macro_rules! concat_string {
    // 匹配多个参数
    ($first:expr $(, $rest:expr)*) => {{
        let capacity = $first.len() $(+ $rest.len() + 1)*;

        // 提前分配容量
        let mut s = String::with_capacity(capacity);

        // 拼接第一个参数
        s.push_str($first);

        $(
            s.push(':');
            s.push_str($rest);
        )*

        s
    }};
}

impl RdSrv {
    pub fn new(db: redis::Client) -> Self {
        Self { db }
    }

    pub async fn add_key(&self, uid: i64, key: &str, limitation: i16) -> Result<(), AppError> {
        let key_set = concat_string!(REDIS_PREFIX, REDIS_KEY);
        let user_key = concat_string!(&key_set, uid.to_string().as_str());
        let mut con = self.db.get_multiplexed_tokio_connection().await?;
        let count: i16 = con.scard(&user_key).await?;
        if count > limitation {
            return Err(AppError::Limit);
        }
        let count: i16 = con.scard(&key_set).await?;
        if count > limitation {
            return Err(AppError::Limit);
        }
        let _: () = con.sadd(user_key, key).await?;
        Ok(con.sadd(key_set, key).await?)
    }

    pub async fn check_key(&self, uid: Option<i64>, key: &str) -> Result<bool, AppError> {
        let key_set = concat_string!(REDIS_PREFIX, REDIS_KEY);
        let mut con = self.db.get_multiplexed_tokio_connection().await?;
        if let Some(uid) = uid {
            let user_key = concat_string!(&key_set, uid.to_string().as_str());
            return Ok(con.sismember(key_set, key).await? && con.sismember(user_key, key).await?);
        }
        Ok(con.sismember(key_set, key).await?)
    }

    pub async fn get_url(&self, key: &str) -> Result<Option<String>, AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_LIST_PREFIX, key);
        let mut con = self.db.get_multiplexed_tokio_connection().await?;
        Ok(con.srandmember(key).await?)
    }

    pub async fn add_url(&self, key: &str, url: &str) -> Result<(), AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_LIST_PREFIX, key);
        let mut con = self.db.get_multiplexed_tokio_connection().await?;
        Ok(con.sadd(key, url).await?)
    }

    pub async fn delete_url(&self, key: &str, url: &str) -> Result<(), AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_LIST_PREFIX, key);
        let mut con = self.db.get_multiplexed_tokio_connection().await?;
        Ok(con.srem(key, url).await?)
    }

    pub async fn get_urls(&self, key: &str) -> Result<Vec<String>, AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_LIST_PREFIX, key);
        let mut con = self.db.get_multiplexed_tokio_connection().await?;
        Ok(con.smembers(key).await?)
    }
    pub async fn set_csrf(&self, csrf: &str) -> Result<(), AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_CSRF);
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut con = self.db.get_multiplexed_tokio_connection().await?;
        let _: () = con.zadd(&key, csrf, current_time + 10 * 60).await?;
        Ok(con.zrembyscore(&key, "-inf", current_time).await?)
    }

    pub async fn check_csrf(&self, csrf: &str) -> Result<bool, AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_CSRF);
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut con = self.db.get_multiplexed_tokio_connection().await?;
        let _: () = con.zrembyscore(&key, "-inf", current_time).await?;
        let rank: Option<isize> = con.zrank(key, csrf).await?;
        Ok(rank.is_some())
    }

    pub async fn get_user_keys(&self, uid: i64) -> Result<Vec<String>, AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_KEY, uid.to_string().as_str());
        let mut con = self.db.get_multiplexed_tokio_connection().await?;
        Ok(con.smembers(key).await?)
    }
}
