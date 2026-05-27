use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Reparacion {
    pub id_reparacion: i32,
    pub id_vehiculo: i32,
    pub id_mecanico: i32,
    pub id_servicio: i32,
    pub fecha_entrada: String, 
    pub estado: String, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaReparacion {
    pub id_mecanico: i32,
    pub id_vehiculo: i32,
    pub id_servicio: i32,
    pub estado: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarReparacion {   
    pub id_reparacion: i32,
    pub estado: String, 
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReparacionDetalle{
    pub id_reparacion: i32,
    pub modelo_vehiculo: String,
    pub nombre_mecanico: String,
    pub descripcion_falla: String,
    pub fecha_entrada: String,
    pub estado: String, 
    pub precio_estimado: f64,
}   