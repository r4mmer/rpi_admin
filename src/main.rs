use envconfig::Envconfig;
use rpi_admin::{ http, worker, config::Config };
// use rpi_admin::{database, http, config};

#[tokio::main]
async fn main() {
    // Setup tracing
    tracing_subscriber::fmt::init();

    // Load config
    let config = Config::init_from_env().unwrap();
    dbg!(config.clone());
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(config.database_file.clone())
        .create_if_missing(true);

    let pool = sqlx::SqlitePool::connect_with(options).await.unwrap();

    // database::crud::init_tables(&pool).await.unwrap();

    // database::crud::digital_timer::DigitalTimerCRUD::insert_one(
    //     &pool,
    //     &database::models::digital_timer::DigitalTimer {
    //         id: None,
    //         name: "Test".to_string(),
    //         pin: 1,
    //         hour: 1,
    //         minute: 1,
    //         duration: 1,
    //         is_enabled: true,
    //     }
    // ).await.unwrap();

    let pool_clone = pool.clone();
    tokio::select! {
        _ = http::serve(config, pool) => {}
        _ = worker::run(pool_clone) => {}
    }
}
