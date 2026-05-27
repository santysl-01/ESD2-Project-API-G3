use sqlx::PgPool;
use crate::models::reparaciones::{Reparacion, NuevaReparacion, ActualizarReparacion, ReparacionDetalle};

pub struct ReparacionRepository {
    pool: PgPool,
}

impl ReparacionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_reparaciones(&self) -> Result<Vec<Reparacion>, sqlx::Error> {

        let reparaciones = sqlx::query_as::<_, Reparacion>(
            "SELECT id_reparacion, id_vehiculo, id_mecanico, id_servicio, fecha_entrada::TEXT, estado FROM reparaciones"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(reparaciones)
    }

    pub async fn crear_reparacion(&self, nueva_reparacion: NuevaReparacion) -> Result<Reparacion, sqlx::Error> {
        let reparacion = sqlx::query_as::<_, Reparacion>(
            "INSERT INTO reparaciones (id_mecanico, id_vehiculo, id_servicio, estado) 
             VALUES ($1, $2, $3, $4) 
             RETURNING id_reparacion, id_vehiculo, id_mecanico, id_servicio, fecha_entrada::TEXT, estado"
        )
        .bind(nueva_reparacion.id_mecanico)
        .bind(nueva_reparacion.id_vehiculo)
        .bind(nueva_reparacion.id_servicio)
        .bind(nueva_reparacion.estado)
        .fetch_one(&self.pool)
        .await?;
        Ok(reparacion)
    }

    pub async fn actualizar_reparacion(&self, actualizacion: ActualizarReparacion) -> Result<Reparacion, sqlx::Error> {
        let reparacion = sqlx::query_as::<_, Reparacion>(
            "UPDATE reparaciones SET estado = $1 
             WHERE id_reparacion = $2 
             RETURNING id_reparacion, id_vehiculo, id_mecanico, id_servicio, fecha_entrada::TEXT, estado"
        )
        .bind(actualizacion.estado)
        .bind(actualizacion.id_reparacion)
        .fetch_one(&self.pool)
        .await?;
        Ok(reparacion)
    }

    pub async fn obtener_reparacion_detalle(&self, id_reparacion: i32) -> Result<ReparacionDetalle, sqlx::Error> {
        let reparacion_detalle = sqlx::query_as::<_, ReparacionDetalle>(
            "SELECT 
                r.id_reparacion, 
                v.modelo AS modelo_vehiculo, 
                m.nombre AS nombre_mecanico, 
                s.descripcion_falla,                
                r.fecha_entrada::TEXT,              
                s.precio_estimado::FLOAT8 AS precio_estimado 
             FROM reparaciones r
             JOIN vehiculos v ON r.id_vehiculo = v.id_vehiculo
             JOIN mecanicos m ON r.id_mecanico = m.id_mecanico
             JOIN servicios s ON r.id_servicio = s.id_servicio
             WHERE r.id_reparacion = $1"
        )
        .bind(id_reparacion)
        .fetch_one(&self.pool)
        .await?;
        Ok(reparacion_detalle)
    }
}