use sqlx::{PgPool,Row};
use crate::models::vehiculo::{Vehiculo,NuevoVehiculo,ActualizarVehiculo};


pub struct VehiculoRepository{
    pool:PgPool,
}

impl VehiculoRepository{
    pub fn new (pool:PgPool) -> Self{
        Self{pool}
    }

    // 1. Obtener todos los vehículos
    pub async fn obtener_vehiculos(&self) -> sqlx::Result<Vec<Vehiculo>> {
        let filas = sqlx::query("SELECT id_vehiculo, placa, marca, id_propietario FROM vehiculo")
            .fetch_all(&self.pool)
            .await?;

        let vehiculos = filas.into_iter().map(|fila| {
            Vehiculo {
                id_vehiculo: fila.get("id_vehiculo"),
                placa: fila.get("placa"),
                marca: fila.get("marca"),
                id_propietario: fila.get("id_propietario"),
            }
        }).collect();

        Ok(vehiculos)
    }

    // 2. Crear un nuevo vehículo
    pub async fn crear_vehiculo(&self, nuevo_vehiculo: NuevoVehiculo) -> sqlx::Result<Vehiculo> {
        let fila = sqlx::query(
            "INSERT INTO vehiculo (placa, marca, id_propietario) VALUES ($1, $2, $3) RETURNING id_vehiculo, placa, marca, id_propietario"
        )
        .bind(nuevo_vehiculo.placa)
        .bind(nuevo_vehiculo.marca)
        .bind(nuevo_vehiculo.id_propietario)
        .fetch_one(&self.pool)
        .await?;

        Ok(Vehiculo {
            id_vehiculo: fila.get("id_vehiculo"),
            placa: fila.get("placa"),
            marca: fila.get("marca"),
            id_propietario: fila.get("id_propietario"),
        })
    }

    // 3. Actualizar un vehículo existente
    pub async fn actualizar_vehiculo(&self, id: i32, vehiculo_actualizado: ActualizarVehiculo) -> sqlx::Result<Vehiculo> {
        let fila = sqlx::query(
            "UPDATE vehiculo SET placa = $1, marca = $2, id_propietario = $3 WHERE id_vehiculo = $4 RETURNING id_vehiculo, placa, marca, id_propietario"
        )
        .bind(vehiculo_actualizado.placa)
        .bind(vehiculo_actualizado.marca)
        .bind(vehiculo_actualizado.id_propietario)
        .bind(id) // Este es el $4 de la consulta
        .fetch_one(&self.pool)
        .await?;

        Ok(Vehiculo {
            id_vehiculo: fila.get("id_vehiculo"),
            placa: fila.get("placa"),
            marca: fila.get("marca"),
            id_propietario: fila.get("id_propietario"),
        })
    }

    // 4. Eliminar un vehículo
    pub async fn eliminar_vehiculo(&self, id: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM vehiculo WHERE id_vehiculo = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}