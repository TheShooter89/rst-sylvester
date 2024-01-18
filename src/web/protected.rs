use askama::Template;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

use crate::users::AuthSession;

use super::home;

#[derive(Template)]
#[template(path = "components/_testing/protected.html")]
struct ProtectedTemplate<'a> {
    username: &'a str,
}

pub fn router() -> Router<()> {
    let home_routes = home::router();

    Router::new()
        .nest("/", home_routes)
        .route("/testing", get(self::get::protected))
}

mod get {
    use super::*;

    pub async fn protected(auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => ProtectedTemplate {
                username: &user.username,
            }
            .into_response(),

            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
