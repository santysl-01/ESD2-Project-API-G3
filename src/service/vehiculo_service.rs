use axum::{extract::{Path, State}, Json};
use sqlx::{PgPool, Pool, Postgres};
use crate::models::vehiculo::{Vehiculo, NuevoVehiculo, ActualizarVehiculo};
use crate::repository::vehiculo_repository::VehiculoRepository;
use axum::http::StatusCode;


pub async fn obtener_vehiculos(State(pool): State<PgPool>) -> Json<Vec<Vehiculo>> {
    let repo = VehiculoRepository::new(pool);
    match repo.obtener_vehiculos().await {
        Ok(vehiculos) => Json(vehiculos),
        Err(_) => Json(vec![]),
    }
} 


pub async fn crear_vehiculo(
    State(pool): State<Pool<Postgres>>,
    Json(nuevo_vehiculo): Json<NuevoVehiculo>,
) -> Result<Json<Vehiculo>, (StatusCode, String)> {
    let repo = VehiculoRepository::new(pool);

    match repo.crear_vehiculo(nuevo_vehiculo).await {
        Ok(vehiculo) => Ok(Json(vehiculo)),
        Err(e) => {
            eprintln!("Error al crear vehículo: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}


pub async fn eliminar_vehiculo(
    State(pool): State<PgPool>, 
    Json(id_vehiculo): Json<i32>
) -> Json<bool> {
    let repo = VehiculoRepository::new(pool);
    match repo.eliminar_vehiculo(id_vehiculo).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}


pub async fn eliminar_vehiculo_por_id(
    State(pool): State<PgPool>,
    Path(id_vehiculo): Path<i32>,
) -> Json<bool> {
    let repo = VehiculoRepository::new(pool);
    match repo.eliminar_vehiculo(id_vehiculo).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}
    pub async fn actualizar_vehiculo(
    State(pool): State<PgPool>,
    Path(id_vehiculo): Path<i32>,
    Json(vehiculo_actualizado): Json<ActualizarVehiculo>,
) -> Json<Vehiculo> {
    let repo = VehiculoRepository::new(pool);
    match repo.actualizar_vehiculo(id_vehiculo, vehiculo_actualizado).await {
        Ok(vehiculo) => Json(vehiculo),
        Err(_) => Json(Vehiculo {
            id_vehiculo: 0,
            placa: "Error al actualizar el vehículo".to_string(),
            marca: String::new(),
            id_propietario: 0,
        }),
    }
}
