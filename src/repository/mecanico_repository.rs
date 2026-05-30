use sqlx::PgPool;
use crate::models::mecanico::{Mecanico, NuevoMecanico, ActualizarMecanico};

pub struct MecanicoRepository {
    pool: PgPool,
}

impl MecanicoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_mecanicos(&self) -> Result<Vec<Mecanico>, sqlx::Error> {
        let mecanicos = sqlx::query_as::<_, Mecanico>(
            "SELECT id_mecanico, nombre, cargo, salario::FLOAT8 FROM mecanicos"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(mecanicos)
    }

    pub async fn crear_mecanico(&self, nuevo: NuevoMecanico) -> Result<Mecanico, sqlx::Error> {
        let mecanico = sqlx::query_as::<_, Mecanico>(
            "INSERT INTO mecanicos (nombre, cargo, salario)
             VALUES ($1, $2, $3)
             RETURNING id_mecanico, nombre, cargo, salario::FLOAT8"
        )
        .bind(nuevo.nombre)
        .bind(nuevo.cargo)
        .bind(nuevo.salario)
        .fetch_one(&self.pool)
        .await?;
        Ok(mecanico)
    }

    pub async fn actualizar_mecanico(&self, actualizacion: ActualizarMecanico) -> Result<Mecanico, sqlx::Error> {
        let mecanico = sqlx::query_as::<_, Mecanico>(
            "UPDATE mecanicos SET nombre = $1, cargo = $2, salario = $3
             WHERE id_mecanico = $4
             RETURNING id_mecanico, nombre, cargo, salario::FLOAT8"
        )
        .bind(actualizacion.nombre)
        .bind(actualizacion.cargo)
        .bind(actualizacion.salario)
        .bind(actualizacion.id_mecanico)
        .fetch_one(&self.pool)
        .await?;
        Ok(mecanico)
    }

    pub async fn eliminar_mecanico(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM mecanicos WHERE id_mecanico = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}