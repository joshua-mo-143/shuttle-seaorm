use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
struct AppState {
    pool: DatabaseConnection,
}

async fn hello_world() -> &'static str {
    "Hello world!"
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_seaorm::Postgres] pool: DatabaseConnection) -> shuttle_axum::ShuttleAxum {
    let state = AppState { pool };
    let router = Router::new().route("/", get(hello_world)).with_state(state);

    Ok(router.into())
}
