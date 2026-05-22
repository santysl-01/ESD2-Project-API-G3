use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

// Tu URL actual del Pooler de Supabase se cargará dinámicamente aquí:
pub fn obtener_url_base_datos() -> String {
    // Carga el archivo .env si existe
    dotenv().ok(); 
    
    // Busca exactamente la variable DATABASE_URL
    env::var("DATABASE_URL").expect("ERROR: DATABASE_URL no está configurada correctamente en el archivo .env")
}

pub async fn crear_pool() -> sqlx::Result<sqlx::Pool<sqlx::Postgres>> {
    let url_base_datos = obtener_url_base_datos();

    // Crea el grupo de conexiones (Pool) hacia Supabase
    PgPoolOptions::new()
        .max_connections(5) // 5 conexiones máximas es ideal para planes gratuitos de Supabase
        .connect(&url_base_datos)
        .await
}