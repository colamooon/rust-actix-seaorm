use std::time::Duration;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use colamooon_api::{models::AppState, router_config::config_router};

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    let mut opt = ConnectOptions::new(conn_spec.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema".into()); // Setting default PostgreSQL schema

    let conn = Database::connect(opt).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    log::info!("starting HTTP server at http://localhost:8080");
    let state = AppState { conn };
    // Start HTTP server
    HttpServer::new(move || {
        // let auth = HttpAuthentication::basic(validator);
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .wrap(middleware::Logger::default())
            // .wrap(auth)
            .app_data(web::Data::new(state.clone()))
            .configure(config_router)
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("welcome!") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
