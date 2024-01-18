use askama::Template;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

use crate::users::AuthSession;

#[derive(Template)]
#[template(path = "components/home/home.html")]
struct HomeTemplate<'a> {
    is_logged: bool,
    username: &'a str,
    title: Option<String>,
    message: Option<String>,
    next: Option<String>,
}

// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub fn router() -> Router<()> {
    Router::new().route("/", get(self::get::home))
    // .route("/about", get(self::get::about))
}

pub mod get {
    use super::*;

    pub async fn home(Query(NextUrl { next }): Query<NextUrl>) -> impl IntoResponse {
        HomeTemplate {
            is_logged: true,
            username: "Pippo",
            title: Some("Info Page".to_string()),
            message: Some("called from home root".to_string()),
            next,
        }
        .into_response()
    }

    pub async fn about(Query(NextUrl { next }): Query<NextUrl>) -> impl IntoResponse {
        HomeTemplate {
            is_logged: true,
            username: "Pippo",
            title: Some("About Page".to_string()),
            message: Some("called from /about".to_string()),
            next,
        }
        .into_response()
    }
}
