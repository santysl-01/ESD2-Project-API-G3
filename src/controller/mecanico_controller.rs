use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::service::mecanico_service::{
    obtener_mecanicos,
    crear_mecanico,
    actualizar_mecanico,
    eliminar_mecanico_por_id,
};

pub fn mecanico_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/mecanicos", get(obtener_mecanicos))
        .route("/api/mecanicos", post(crear_mecanico))
        .route("/api/mecanicos", put(actualizar_mecanico))
        .route("/api/mecanicos/{id}", delete(eliminar_mecanico_por_id))
        .with_state(pool)
}