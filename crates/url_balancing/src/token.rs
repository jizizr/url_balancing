use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ring::rand::{SecureRandom, SystemRandom};

/// 生成指定长度的URL安全Token
pub fn new_token() -> String {
    let rng = SystemRandom::new();
    let mut random_bytes = vec![0u8; 32];
    rng.fill(&mut random_bytes)
        .expect("Failed to generate random bytes");
    URL_SAFE_NO_PAD.encode(&random_bytes)
}
