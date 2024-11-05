use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use oauth2::{basic::BasicErrorResponseType, RequestTokenError, StandardErrorResponse};
use serde::Serialize;
use strum_macros::EnumDiscriminants;
use thiserror::Error;

#[repr(i8)]
#[derive(Error, Debug, EnumDiscriminants)]
#[strum_discriminants(name(AppErrorKind))]
pub enum AppError {
    #[error("404 Not Found")]
    HTTPNotFound,
    #[error("请求令牌错误: {0}")]
    OAuth2RequestToken(
        #[from]
        RequestTokenError<
            oauth2::reqwest::AsyncHttpClientError,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
    #[error("请求错误: {0}")]
    Request(#[from] reqwest::Error),
    #[error("url解析失败")]
    UrlParse(#[from] url::ParseError),
    #[error("Redis错误: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("MySQL错误: {0}")]
    MySQL(#[from] sea_orm::DbErr),
    #[error("Key not found")]
    KeyNotFound,
    #[error("state过期或不存在")]
    StateNotFound,
    #[error("token异常: {0}")]
    Token(#[from] jsonwebtoken::errors::Error),
    #[error("未认证")]
    Unauthorized,
    #[error("无效的响应")]
    Invalid,
    #[error("数量达到上限")]
    Limit,
    #[error("未知错误")]
    Unknown,
}
impl From<&AppError> for i8 {
    fn from(error: &AppError) -> i8 {
        let kind: AppErrorKind = AppErrorKind::from(error); // 获取判别器
        kind as i8
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    code: i8,
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match &self {
            AppError::HTTPNotFound => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
            _ => (
                StatusCode::OK,
                Json(ErrorResponse {
                    code: <&AppError as Into<i8>>::into(&self),
                    error: self.to_string(),
                }),
            )
                .into_response(),
        }
    }
}
