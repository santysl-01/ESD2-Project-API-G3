use axum::{routing ::{delete,get,post,put},
Router,
};

use sqlx::PgPool; 
use crate::service::vehiculo_service::{
     obtener_vehiculos,crear_vehiculo,eliminar_vehiculo,eliminar_vehiculo_por_id,actualizar_vehiculo
};

pub fn vehiculo_router(pool:PgPool) -> Router {
Router ::new() 
.route("/vehiculo", get(obtener_vehiculos))
.route("/vehiculo", post(crear_vehiculo))
.route("/vehiculo", delete(eliminar_vehiculo))
.route("/vehiculo/{id}", delete(eliminar_vehiculo_por_id))
.route("/vehiculo", put(actualizar_vehiculo))
.with_state(pool)
}