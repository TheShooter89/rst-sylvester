use askama::Template;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

use crate::users::AuthSession;

#[derive(Template)]
#[template(path = "components/about/about.html")]
struct AboutTemplate<'a> {
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
    Router::new().route("/", get(self::get::about))
}

pub mod get {
    use super::*;

    pub async fn about(auth_session: AuthSession) -> impl IntoResponse {
        let is_logged = match auth_session.user {
            Some(_) => true,
            None => false,
        };

        let username = match auth_session.user {
            Some(user) => user.username.clone(),
            None => "Pippo".to_string(),
        };

        AboutTemplate {
            is_logged: is_logged,
            username: &username,
            title: Some("About Page".to_string()),
            message: Some("called from /about".to_string()),
            next: None,
        }
        .into_response()
    }
}
