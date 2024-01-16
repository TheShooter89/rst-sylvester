use askama::Template;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

use crate::users::AuthSession;

#[derive(Template)]
#[template(path = "protected.html")]
struct ProtectedTemplate<'a> {
    username: &'a str,
}

#[derive(Template)]
#[template(path = "home/home.html")]
struct ProtectedTemplateAlt<'a> {
    is_logged: bool,
    username: &'a str,
    title: Option<String>,
    message: Option<String>,
    next: Option<String>,
}

pub fn router() -> Router<()> {
    Router::new().route("/", get(self::get::home))
}

mod get {
    use super::*;

    pub async fn home(auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => ProtectedTemplate {
                username: &user.username,
            }
            .into_response(),

            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn info() -> impl IntoResponse {
        ProtectedTemplateAlt {
            is_logged: true,
            username: "Pippo",
            title: Some("Info Page".to_string()),
            message: Some("Some message".to_string()),
            next: None,
        }
        .into_response()
        // "test string response".to_string().into_response()
    }
}
