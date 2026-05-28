use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::propietario::{Propietario, NuevoPropietario, ActualizarPropietario};
use crate::repository::propietario_repository::PropietarioRepository;

pub async fn obtener_propietarios(State(pool): State<PgPool>) -> Json<Vec<Propietario>> {
    let repo = PropietarioRepository::new(pool);
    match repo.obtener_propietarios().await {
        Ok(propietarios) => Json(propietarios),
        Err(_) => Json(vec![]),
    }
}

pub async fn crear_propietario(
    State(pool): State<PgPool>,
    Json(nuevo): Json<NuevoPropietario>,
) -> Json<Propietario> {
    let repo = PropietarioRepository::new(pool);
    match repo.crear_propietario(nuevo).await {
        Ok(propietario) => Json(propietario),
        Err(_) => Json(Propietario {
            id_propietario: 0,
            nombre: String::new(),
            dui: None,
            telefono: None,
        }),
    }
}

pub async fn actualizar_propietario(
    State(pool): State<PgPool>,
    Json(actualizacion): Json<ActualizarPropietario>,
) -> Json<Propietario> {
    let repo = PropietarioRepository::new(pool);
    match repo.actualizar_propietario(actualizacion).await {
        Ok(propietario) => Json(propietario),
        Err(_) => Json(Propietario {
            id_propietario: 0,
            nombre: String::new(),
            dui: None,
            telefono: None,
        }),
    }
}

pub async fn eliminar_propietario_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Json<bool> {
    let repo = PropietarioRepository::new(pool);
    match repo.eliminar_propietario(id).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}