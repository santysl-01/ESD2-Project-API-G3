use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Servicio {
    pub id_servicio: i32,
    pub descripcion_falla: String,
    pub precio_estimado: Option<rust_decimal::Decimal>,
}

#[derive(Serialize, Deserialize)]
pub struct NuevoServicio {
    pub descripcion_falla: String,
    pub precio_estimado: Option<rust_decimal::Decimal>,
}

#[derive(Serialize, Deserialize)]
pub struct ActualizarServicio {
    pub id_servicio: i32,
    pub descripcion_falla: String,
    pub precio_estimado: Option<rust_decimal::Decimal>,
}
