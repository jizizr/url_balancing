use std::env;

pub fn establish_connection() -> redis::Client {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_else(|_| "".to_string());

    let redis_url_with_password = if redis_password.is_empty() {
        redis_url
    } else {
        let url = url::Url::parse(&redis_url).expect("Invalid Redis URL");
        let mut url_with_password = url.clone();
        url_with_password
            .set_password(Some(&redis_password))
            .expect("Failed to set password");
        url_with_password.to_string()
    };

    redis::Client::open(redis_url_with_password).expect("Invalid Redis client")
}
