use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::reparaciones::{ActualizarReparacion, NuevaReparacion, Reparacion, ReparacionDetalle};
use crate::repository::ReparacionRepository;

pub async fn obtener_reparaciones(State(pool): State<PgPool>) -> Json<Vec<Reparacion        >> {
    let reparaciones = ReparacionRepository::new(pool);
    match reparaciones.obtener_reparaciones().await {
        Ok(reparaciones) => Json(reparaciones),
        Err(_) => Json(vec![]), // Manejo de errores simplificado    
    }
}

pub async fn crear_reparacion(State(pool): State<PgPool>, Json(nueva_reparacion): Json<NuevaReparacion>) -> Json<Reparacion> {
    let reparaciones = ReparacionRepository::new(pool);
    match reparaciones.crear_reparacion(nueva_reparacion).await {
        Ok(reparacion) => Json(reparacion),
        Err(_) => Json(Reparacion { // Manejo de errores simplificado
            id_reparacion: 0,
            id_vehiculo: 0,
            id_mecanico: 0,
            id_servicio: 0,
            fecha_entrada: String::new(),
            estado: String::new(),
        }),
    }
}

pub async fn actualizar_reparacion(State(pool): State<PgPool>, Json(actualizar_reparacion): Json<ActualizarReparacion>) -> Json<Reparacion> {
    let reparaciones = ReparacionRepository::new(pool);
    match reparaciones.actualizar_reparacion(actualizar_reparacion).await {
        Ok(reparacion) => Json(reparacion),
        Err(_) => Json(Reparacion { // Manejo de errores simplificado
            id_reparacion: 0,
            id_vehiculo: 0,
            id_mecanico: 0,
            id_servicio: 0,
            fecha_entrada: String::new(),
            estado: String::new(),
        }),
    }
}

pub async fn obtener_reparacion_detalle(State(pool): State<PgPool>, Path(id_reparacion): Path<i32>) -> Json<ReparacionDetalle> {
    let reparaciones = ReparacionRepository::new(pool);
    match reparaciones.obtener_reparacion_detalle(id_reparacion).await {
        Ok(reparacion_detalle) => Json(reparacion_detalle),
        Err(_) => Json(ReparacionDetalle { // Manejo de errores simplificado
            id_reparacion: 0,
            modelo_vehiculo: String::new(),
            nombre_mecanico: String::new(),
            descripcion_falla: String::new(),
            fecha_entrada: String::new(),
            estado: String::new(),
            precio_estimado: 0.0,
        }),
    }
}
    