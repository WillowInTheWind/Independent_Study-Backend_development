use axum::{middleware, Router};
use axum::routing::{any, delete, get, post};
use http::header::CONTENT_TYPE;
use http::Method;
use crate::{defaultroutes, loginroutes, MXroutes, UserRoutes};
use crate::middlewares::auth;
use crate::state::AppState;
use tower_http::cors::{Any, CorsLayer};

pub fn router(app_state: AppState) -> Router {
    let cors = CorsLayer::permissive();
    let mx_routes = Router::new()
        .route("/", get(MXroutes::get_all_mxs))
        .route("/create", post(MXroutes::post_mx))
        .route("/delete", delete(MXroutes::delete_mx))
        .route("/mine", get(MXroutes::get_users_mxs));
    let user_routes = Router::new()
        .route("/", get(UserRoutes::get_all_users))
        .route("/:id", get(UserRoutes::get_user_by_id))
        // .route("/:user_property", get(UserRoutes::get_user_by))
        .route("/delete", delete(UserRoutes::delete_user));
    let auth_routes = Router::new()
        .route("/logout", get(loginroutes::logout))
        .route("/login", get(loginroutes::login))
        .route("/authorized", get(loginroutes::login_authorized));
    Router::new()
        .route("/", get(defaultroutes::root))
        .nest("/morningexercises", mx_routes)
        .layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .nest("/users", user_routes)
        .nest("/auth", auth_routes)
        .fallback(defaultroutes::error_404)
        .with_state(app_state)
        .layer(cors)
}
