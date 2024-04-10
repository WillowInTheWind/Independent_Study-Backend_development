use axum::{middleware, Router};
use axum::routing::{any, delete, get, post};
use http::header::CONTENT_TYPE;
use http::{HeaderValue, Method};
use crate::{defaultroutes, loginroutes, middlewares, MXroutes, UserRoutes};
use crate::middlewares::auth;
use crate::state::AppState;
use tower_http::cors::{Any, CorsLayer};

pub fn router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap());
    let mx_routes = Router::new()
        .route("/", get(MXroutes::get_all_mxs))
        .route("/approve", post(MXroutes::approve_mx))
        .route("/create", post(MXroutes::post_mx))
        .route("/delete", delete(MXroutes::delete_mx))
        .route("/mine", get(MXroutes::get_users_mxs));
    let user_routes = Router::new()
        .route("/currentuser", get(UserRoutes::current_user))
        .route("/setphonenumber", post(UserRoutes::set_user_number))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .route("/", get(UserRoutes::get_all_users))
        .route("/getbyid/:id", get(UserRoutes::get_user_by_id))
        .route("/getuserby/:user_property", get(UserRoutes::get_user_by))
        .route("/delete", delete(UserRoutes::delete_user));
    let auth_routes = Router::new()
        .route("/logout", get(loginroutes::logout))
        .route("/login", get(loginroutes::login))
        .route("/authorized", get(loginroutes::login_authorized));
    let routes = Router::new()
        .route("/", get(defaultroutes::root))
        .nest("/morningexercises", mx_routes)
        .layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .nest("/users", user_routes)
        .nest("/auth", auth_routes)
        .fallback(defaultroutes::error_404)
        .with_state(app_state);

    Router::new().nest("/api", routes).layer(cors)
}
