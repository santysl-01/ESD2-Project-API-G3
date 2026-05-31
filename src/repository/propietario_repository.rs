use sqlx::PgPool;
use crate::models::propietario::{Propietario, NuevoPropietario, ActualizarPropietario};

pub struct PropietarioRepository {
    pool: PgPool,
}

impl PropietarioRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_propietarios(&self) -> Result<Vec<Propietario>, sqlx::Error> {
        let propietarios = sqlx::query_as::<_, Propietario>(
            "SELECT id_propietario, nombre, dui, telefono FROM propietarios"
        )
        
        .fetch_all(&self.pool)
        
        .await?;
        Ok(propietarios)
    }

    pub async fn crear_propietario(&self, nuevo: NuevoPropietario) -> Result<Propietario, sqlx::Error> {
        let propietario = sqlx::query_as::<_, Propietario>(
            "INSERT INTO propietarios (nombre, dui, telefono)
             VALUES ($1, $2, $3)
             RETURNING id_propietario, nombre, dui, telefono"
        )
        .bind(nuevo.nombre)
        .bind(nuevo.dui)
        .bind(nuevo.telefono)
        .persistent(false)
        .fetch_one(&self.pool)
        .await?;
        Ok(propietario)
    }

    pub async fn actualizar_propietario(&self, actualizacion: ActualizarPropietario) -> Result<Propietario, sqlx::Error> {
        let propietario = sqlx::query_as::<_, Propietario>(
            "UPDATE propietarios SET nombre = $1, dui = $2, telefono = $3
             WHERE id_propietario = $4
             RETURNING id_propietario, nombre, dui, telefono"
        )
        .bind(actualizacion.nombre)
        .bind(actualizacion.dui)
        .bind(actualizacion.telefono)
        .bind(actualizacion.id_propietario)
        .persistent(false)    
        .fetch_one(&self.pool)
        .await?;
        Ok(propietario)
    }

    pub async fn eliminar_propietario(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM propietarios WHERE id_propietario = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}