mod service;
mod utils;
mod models;
mod controller;
mod config;
mod repository;

use dotenvy::dotenv;

// cambiar por los controllers que tengas
use controller::vehiculo_controller::vehiculo_router;
use controller::reparaciones_controller::reparacion_router;
use controller::servicios_controller::servicios_router;
use controller::propietario_controller::propietario_router  ;

use config::config::crear_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let direccion = "127.0.0.1:3000";

    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor escuchando en http://{direccion}");

    let pool = crear_pool()
        .await
        .expect("No se pudo conectar a la base de datos");

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al iniciar el servidor");
}

// unir todos los routers de la API
fn unificar_routers(pool: sqlx::PgPool) -> axum::Router {
    axum::Router::new()
        .merge(vehiculo_router(pool.clone()))
        .merge(reparacion_router(pool.clone()))
        .merge(propietario_router(pool.clone()))
        .merge(servicios_router(pool))
        
}
