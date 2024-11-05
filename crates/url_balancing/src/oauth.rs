use std::sync::Arc;

use crate::{error::AppError, handler::CommonResponse, jwt::create_jwt, state::AppState};
use axum::{
    extract::Query,
    http::{header::SET_COOKIE, HeaderMap},
    response::Redirect,
    Extension, Json,
};
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    TokenResponse as _, TokenUrl,
};
use serde::{Deserialize, Serialize};

pub fn oauth2_client() -> Result<BasicClient, AppError> {
    let client_id = ClientId::new(std::env::var("OAUTH_CLIENT_ID").unwrap());
    let client_secret = ClientSecret::new(std::env::var("OAUTH_CLIENT_SECRET").unwrap());
    let auth_url = AuthUrl::new("https://connect.linux.do/oauth2/authorize".to_string())?;
    let token_url = TokenUrl::new("https://connect.linux.do/oauth2/token".to_string())?;
    Ok(BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    ))
}

pub async fn linuxdo_auth(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Redirect, AppError> {
    let (auth_url, csrf_token) = state
        .oauth2_client
        .authorize_url(CsrfToken::new_random)
        .url();
    state.set_csrf(csrf_token.secret()).await?;
    Ok(Redirect::to(auth_url.as_ref()))
}
#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LinuxDoUser {
    pub id: i64,
    pub name: String,
    pub avatar_url: String,
}

#[derive(Serialize)]
pub struct LinuxAuthResponse {
    token: String,
}

pub async fn linuxdo_authorized(
    Query(query): Query<AuthRequest>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<HeaderMap, AppError> {
    if !state.check_csrf(&query.state).await? {
        return Err(AppError::StateNotFound);
    }
    let token = state
        .oauth2_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(oauth2::reqwest::async_http_client)
        .await?;
    let client = reqwest::Client::new();
    let user: LinuxDoUser = client
        .get("https://connect.linux.do/api/user")
        .bearer_auth(token.access_token().secret())
        .send()
        .await?
        .json()
        .await?;
    let jwtoken = create_jwt(user)?;
    let cookie = format!(
        "jwt={jwtoken}; Path=/; HttpOnly; SameSite=Lax; Max-Age={max_age}",
        jwtoken = jwtoken,
        max_age = 3600 * 24 * 30
    );
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().map_err(|_| AppError::Invalid)?);
    Ok(headers)
}

pub async fn user_info(
    Extension(user): Extension<LinuxDoUser>,
) -> Json<CommonResponse<LinuxDoUser>> {
    Json(CommonResponse {
        code: 0,
        data: Some(user),
    })
}
