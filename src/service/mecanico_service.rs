use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::mecanico::{Mecanico, NuevoMecanico, ActualizarMecanico};
use crate::repository::mecanico_repository::MecanicoRepository;

pub async fn obtener_mecanicos(State(pool): State<PgPool>) -> Json<Vec<Mecanico>> {
    let repo = MecanicoRepository::new(pool);
    match repo.obtener_mecanicos().await {
        Ok(mecanicos) => Json(mecanicos),
        Err(_) => Json(vec![]),
    }
}

pub async fn crear_mecanico(
    State(pool): State<PgPool>,
    Json(nuevo): Json<NuevoMecanico>,
) -> Json<Mecanico> {
    let repo = MecanicoRepository::new(pool);
    match repo.crear_mecanico(nuevo).await {
        Ok(mecanico) => Json(mecanico),
        Err(_) => Json(Mecanico {
            id_mecanico: 0,
            nombre: String::new(),
            cargo: None,
            salario: None,
        }),
    }
}

pub async fn actualizar_mecanico(
    State(pool): State<PgPool>,
    Json(actualizacion): Json<ActualizarMecanico>,
) -> Json<Mecanico> {
    let repo = MecanicoRepository::new(pool);
    match repo.actualizar_mecanico(actualizacion).await {
        Ok(mecanico) => Json(mecanico),
        Err(_) => Json(Mecanico {
            id_mecanico: 0,
            nombre: String::new(),
            cargo: None,
            salario: None,
        }),
    }
}

pub async fn eliminar_mecanico_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Json<bool> {
    let repo = MecanicoRepository::new(pool);
    match repo.eliminar_mecanico(id).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}