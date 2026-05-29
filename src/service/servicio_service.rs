use axum::{extract::{State, Path}, Json};
use sqlx::PgPool;

use crate::models::servicio::{Servicio, NuevoServicio, ActualizarServicio};
use crate::repository::servicio_repository::ServicioRepository;

pub async fn obtener_servicios(State(pool): State<PgPool>) -> Json<Vec<Servicio>> {
    let repo = ServicioRepository::new(pool);

    match repo.obtener_servicios().await {
        Ok(data) => Json(data),
        Err(e) => {
            eprintln!("ERROR AL OBTENER SERVICIOS: {:?}", e);
            Json(vec![])
        }
    }
}

pub async fn crear_servicio(
    State(pool): State<PgPool>,
    Json(nuevo): Json<NuevoServicio>,
) -> Json<Servicio> {
    let repo = ServicioRepository::new(pool);

    match repo.crear_servicio(nuevo).await {
        Ok(data) => Json(data),
        Err(e) => {
            eprintln!("ERROR AL CREAR SERVICIO: {:?}", e);
            Json(Servicio {
                id_servicio: 0,
                descripcion_falla: String::new(),
                precio_estimado: None,
            })
        }
    }
}

pub async fn actualizar_servicio(
    State(pool): State<PgPool>,
    Json(actualizacion): Json<ActualizarServicio>,
) -> Json<Servicio> {
    let repo = ServicioRepository::new(pool);

    match repo.actualizar_servicio(actualizacion).await {
        Ok(data) => Json(data),
        Err(e) => {
            eprintln!("ERROR AL ACTUALIZAR SERVICIO: {:?}", e);
            Json(Servicio {
                id_servicio: 0,
                descripcion_falla: String::new(),
                precio_estimado: None,
            })
        }
    }
}

pub async fn eliminar_servicio_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Json<&'static str> {
    let repo = ServicioRepository::new(pool);

    match repo.eliminar_servicio(id).await {
        Ok(_) => Json("Servicio eliminado correctamente"),
        Err(e) => {
            eprintln!("ERROR AL ELIMINAR SERVICIO: {:?}", e);
            Json("Error al eliminar servicio")
        }
    }
}
