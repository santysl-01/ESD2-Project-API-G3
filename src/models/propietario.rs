use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Propietario {
    pub id_propietario: i32,
    pub nombre: String,
    pub dui: Option<String>,
    pub telefono: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoPropietario {
    pub nombre: String,
    pub dui: Option<String>,
    pub telefono: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarPropietario {
    pub id_propietario: i32,
    pub nombre: String,
    pub dui: Option<String>,
    pub telefono: Option<String>,
}