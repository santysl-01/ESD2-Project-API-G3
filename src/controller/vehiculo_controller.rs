use axum::{routing ::{delete,get,post,put},
Router,
};

use sqlx::PgPool; 
use crate::service::vehiculo_service::{
     obtener_vehiculos,crear_vehiculo,eliminar_vehiculo,eliminar_vehiculo_por_id,actualizar_vehiculo
};

pub fn vehiculo_router(pool:PgPool) -> Router {
Router ::new() 
.route("/api/vehiculos", get(obtener_vehiculos))
.route("/api/vehiculos", post(crear_vehiculo))
.route("/api/vehiculos", delete(eliminar_vehiculo))
.route("/api/vehiculos/{id}", delete(eliminar_vehiculo_por_id))
.route("/api/vehiculos/{id}", put(actualizar_vehiculo))
.with_state(pool)
}