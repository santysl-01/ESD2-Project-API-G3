use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;

use crate::service::servicio_service::{
    obtener_servicios,
    crear_servicio,
    actualizar_servicio,
    eliminar_servicio_por_id,
};

pub fn servicios_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/servicios", get(obtener_servicios))
        .route("/api/servicios", post(crear_servicio))
        .route("/api/servicios/{id}", put(actualizar_servicio))
        .route("/api/servicios/{id}", delete(eliminar_servicio_por_id))
        .with_state(pool)
}
