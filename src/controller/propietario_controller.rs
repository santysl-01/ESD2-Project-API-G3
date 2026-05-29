use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::service::propietario_service::{
    obtener_propietarios,
    crear_propietario,
    actualizar_propietario,
    eliminar_propietario_por_id,
};

pub fn propietario_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/propietarios", get(obtener_propietarios))
        .route("/api/propietarios", post(crear_propietario))
        .route("/api/propietarios", put(actualizar_propietario))
        .route("/api/propietarios/{id}", delete(eliminar_propietario_por_id))
        .with_state(pool)
}