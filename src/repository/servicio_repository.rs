use sqlx::PgPool;
use crate::models::servicio::{Servicio, NuevoServicio, ActualizarServicio};

pub struct ServicioRepository {
    pool: PgPool,
}

impl ServicioRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_servicios(&self) -> Result<Vec<Servicio>, sqlx::Error> {
        let servicios = sqlx::query_as::<_, Servicio>(
            "SELECT id_servicio, descripcion_falla, precio_estimado FROM servicios"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(servicios)
    }

    pub async fn crear_servicio(&self, nuevo: NuevoServicio) -> Result<Servicio, sqlx::Error> {
        let servicio = sqlx::query_as::<_, Servicio>(
            "INSERT INTO servicios (descripcion_falla, precio_estimado)
             VALUES ($1, $2)
             RETURNING id_servicio, descripcion_falla, precio_estimado"
        )
        .bind(nuevo.descripcion_falla)
        .bind(nuevo.precio_estimado)
        .fetch_one(&self.pool)
        .await?;

        Ok(servicio)
    }

    pub async fn actualizar_servicio(&self, actualizacion: ActualizarServicio) -> Result<Servicio, sqlx::Error> {
        let servicio = sqlx::query_as::<_, Servicio>(
            "UPDATE servicios
             SET descripcion_falla = $1, precio_estimado = $2
             WHERE id_servicio = $3
             RETURNING id_servicio, descripcion_falla, precio_estimado"
        )
        .bind(actualizacion.descripcion_falla)
        .bind(actualizacion.precio_estimado)
        .bind(actualizacion.id_servicio)
        .fetch_one(&self.pool)
        .await?;

        Ok(servicio)
    }

    pub async fn eliminar_servicio(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM servicios WHERE id_servicio = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
