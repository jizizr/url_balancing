use crate::{error::AppError, oauth::LinuxDoUser, state::AppState, token};
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::Redirect,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const LIMITATION: i16 = 100;

pub async fn url_balancing(
    Path(key): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Redirect, AppError> {
    let backend_url = state.get_url(&key).await?;
    match backend_url {
        Some(url) => Ok(Redirect::temporary(&url)),
        None => Err(AppError::HTTPNotFound),
    }
}

#[derive(Deserialize)]
pub struct AddUrlRequest {
    url: String,
}

#[derive(Serialize)]
pub struct CommonResponse<S: Serialize> {
    pub code: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<S>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: i8,
    pub error: String,
}

pub async fn add_url(
    Path(key): Path<String>,
    Extension(user): Extension<LinuxDoUser>,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<AddUrlRequest>,
) -> Result<(StatusCode, Json<CommonResponse<()>>), AppError> {
    if !state.check_key(Some(user.id), &key).await.unwrap() {
        return Err(AppError::Invalid);
    }

    state.add_url(&key, &payload.url).await?;

    Ok((
        StatusCode::OK,
        Json(CommonResponse {
            code: 0,
            data: None,
        }),
    ))
}

pub async fn create_key(
    Extension(user): Extension<LinuxDoUser>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<CommonResponse<String>>), AppError> {
    let key = token::new_token();
    if state.check_key(Some(user.id), &key).await? {
        return Err(AppError::Invalid);
    }
    state.add_key(user.id, &key, LIMITATION).await?;
    Ok((
        StatusCode::OK,
        Json(CommonResponse {
            code: 0,
            data: Some(key),
        }),
    ))
}

pub async fn delete_url(
    Path(key): Path<String>,
    Extension(user): Extension<LinuxDoUser>,
    Extension(state): Extension<Arc<AppState>>,
    Json(url): Json<AddUrlRequest>,
) -> Result<(StatusCode, Json<CommonResponse<()>>), AppError> {
    if !state.check_key(Some(user.id), &key).await.unwrap() {
        return Err(AppError::Invalid);
    }

    state.delete_url(&key, &url.url).await?;

    Ok((
        StatusCode::OK,
        Json(CommonResponse {
            code: 0,
            data: None,
        }),
    ))
}

pub async fn get_urls(
    Path(key): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<CommonResponse<Vec<String>>>), AppError> {
    if !state.check_key(None, &key).await.unwrap() {
        return Err(AppError::Invalid);
    }

    let urls = state.get_urls(&key).await?;

    Ok((
        StatusCode::OK,
        Json(CommonResponse {
            code: 0,
            data: Some(urls),
        }),
    ))
}

pub async fn get_keys(
    Extension(user): Extension<LinuxDoUser>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<CommonResponse<Vec<String>>>), AppError> {
    let tokens = state.get_user_keys(user.id).await?;

    Ok((
        StatusCode::OK,
        Json(CommonResponse {
            code: 0,
            data: Some(tokens),
        }),
    ))
}
