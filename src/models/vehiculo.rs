use serde::{Deserialize,Serialize};
use sqlx::FromRow;

#[derive(Debug,Clone, Serialize,Deserialize,FromRow,PartialEq)]
pub struct Vehiculo{
    pub id_vehiculo:i32,
    pub placa:String,
    pub marca:String,
    pub  id_propietario :i32,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct NuevoVehiculo {
    pub placa: String,
    pub marca: String,
    pub id_propietario: i32,
}


#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct ActualizarVehiculo {
    pub placa: String,
    pub marca: String,
    pub id_propietario: i32,
}