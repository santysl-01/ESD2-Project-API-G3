use axum::{
    extract::{Path,State},
    Json
};
use sqlx::PgPool;
use crate::models::vehiculo::Vehiculo;
 use crate::repository::vehiculo_vehiculo::VehiculoRepository;


 // 1. Obtener todos los vehículos
pub async fn obtener_vehiculos(State(pool): State<PgPool>) -> Json<Vec<Vehiculo>> {
    let repo = VehiculoRepository::new(pool);
    match repo.obtener_vehiculos().await {
        Ok(vehiculos) => Json(vehiculos),
        Err(_) => Json(vec![]),
    }
}

// 2. Crear un vehículo
pub async fn crear_vehiculo(
    State(pool): State<PgPool>,
    Json(nuevo_vehiculo): Json<NuevoVehiculo>,
) -> Json<Vehiculo> {
    let repo = VehiculoRepository::new(pool);
    match repo.crear_vehiculo(nuevo_vehiculo).await {
        Ok(vehiculo) => Json(vehiculo),
        Err(_) => Json(Vehiculo {
            id_vehiculo: 0,
            placa: "Error al crear el vehículo".to_string(),
            marca: String::new(),
            id_propietario: 0,
        }),
    }
}

// 3. Eliminar vehículo pasando el ID por el Body (JSON)
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

// 4. Eliminar vehículo pasando el ID por la URL (Path)
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