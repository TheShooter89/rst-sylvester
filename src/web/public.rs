use axum::Router;

use super::about;

pub fn router() -> Router<()> {
    let about_routes = about::router();

    Router::new().nest("/", about_routes)
}
