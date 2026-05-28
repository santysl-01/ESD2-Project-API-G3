use axum::{Router, routing::get, extract::State, Json};
use sqlx::PgPool;
use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct Servicio {
    pub id_servicio: i32,
    pub descripcion_falla: String,
    pub precio_estimado: Option<f64>,
}

pub fn servicios_router(pool: PgPool) -> Router {
    Router::new()
        .route("/servicios", get(listar_servicios))
        .with_state(pool)
}

async fn listar_servicios(
    State(pool): State<PgPool>,
) -> Json<Vec<Servicio>> {

    let servicios = sqlx::query_as::<_, Servicio>(
        r#"
        SELECT id_servicio, descripcion_falla, precio_estimado
        FROM servicios
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Json(servicios)
}
