mod notifier;
mod subscribe_handler;
use actix_web::{App, HttpServer};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_path = env::var("DATABASE_URL").expect("DATABASE_URL can't be blank");
    let api_key = env::var("FCM_API_KEY").expect("FCM_API_KEY can't be blank");
    let notify_period = match env::var("NOTIFY_PERIOD_SECS") {
        Ok(period) => period.parse::<u32>().expect("NOTIFY_PERIOD_SECS has to be an integer"),
        _ => 1800
    };

    // Create database
    sqlx::Sqlite::create_database(&db_path)
        .await
        .expect("error while creating db");
    let db_pool = SqlitePoolOptions::new()
        .max_connections(300)
        .connect(&db_path)
        .await
        .expect("error while connecting to sqlite db");

    let fcm_client = fcm::Client::new();

    let pool = db_pool.clone();
    let notifier = notifier::Notifier::new(pool, fcm_client, api_key, notify_period);
    tokio::spawn(async move {
        let res = notifier.start_loop().await;
        if res.is_err() {
            println!("error while running the loop: {:?}", res);
        }
    });

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .service(subscribe_handler::subscribe)
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
