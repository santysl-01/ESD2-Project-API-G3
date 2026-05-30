use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Mecanico {
    pub id_mecanico: i32,
    pub nombre: String,
    pub cargo: Option<String>,
    pub salario: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoMecanico {
    pub nombre: String,
    pub cargo: Option<String>,
    pub salario: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarMecanico {
    pub id_mecanico: i32,
    pub nombre: String,
    pub cargo: Option<String>,
    pub salario: Option<f64>,
}