use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::service::reparaciones_service::{
    obtener_reparaciones,
    crear_reparacion,
    actualizar_reparacion,
    obtener_reparacion_detalle,
};

pub fn reparacion_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/reparaciones", get(obtener_reparaciones))
        .route("/api/reparaciones", post(crear_reparacion))
        .route("/api/reparaciones/{id}", put(actualizar_reparacion))
        .route("/api/reparaciones/{id}/detalle", get(obtener_reparacion_detalle))
        .with_state(pool)
}