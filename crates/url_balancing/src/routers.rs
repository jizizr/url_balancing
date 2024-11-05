use std::sync::Arc;

use crate::{dao, handler::*, middleware, oauth::*, state};
use axum::{
    routing::{delete, get, post},
    Extension, Router,
};
use tower::ServiceBuilder;

pub async fn init_router() -> Router {
    let mdb_conn = dao::mysql::init::establish_connection().await.unwrap();
    let mdb = dao::mysql::db::DbSrv::new(mdb_conn);
    let rdb_conn = dao::redis::init::establish_connection();
    let rdb = dao::redis::db::RdSrv::new(rdb_conn);
    let oauth2_client = oauth2_client().unwrap();
    let state = state::AppState {
        mdb,
        rdb,
        oauth2_client,
    };
    let cookie_layer = ServiceBuilder::new().layer(axum::middleware::from_fn(middleware::jwt_auth));
    let routes_with_auth = Router::new()
        .route("/key", post(create_key))
        .route("/key", get(get_keys))
        .route("/:key/url", post(add_url))
        .route("/:key/url", delete(delete_url))
        .route("/user", get(user_info))
        .layer(cookie_layer);
    let router_without_auth = Router::new()
        .route("/:key", post(url_balancing).get(url_balancing))
        .route("/:key/urls", get(get_urls))
        .route("/auth/linuxdo", get(linuxdo_auth))
        .route("/auth/authorized", get(linuxdo_authorized));
    Router::new()
        .merge(routes_with_auth)
        .merge(router_without_auth)
        .layer(Extension(Arc::new(state)))
}
