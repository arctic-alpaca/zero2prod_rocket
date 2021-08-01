use rocket::{fairing, routes};
use rocket::{Build, Rocket};
use rocket_db_pools::Database;

use crate::routes::{health_check_route, subscribe};
use rocket::error;
use rocket::fairing::AdHoc;
use sqlx::Executor;
use sqlx::{Connection, PgConnection};
use uuid::Uuid;

#[derive(Database)]
#[database("newsletter")]
pub struct Newsletter(rocket_db_pools::sqlx::PgPool);

pub fn run() -> Rocket<Build> {
    //let figment = rocket::Config::figment().merge(("port", 0));
    //rocket::custom(figment).mount("/health_check", routes![health_check])
    rocket::build()
        .attach(Newsletter::init())
        .mount("/health_check", routes![health_check_route])
        .mount("/subscriptions", routes![subscribe])
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Newsletter::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

/// Create a database with a random name. Then start Rocket with this database as `newsletter` database.
pub async fn run_with_random_database_name() -> Rocket<Build> {
    let db_url = String::from("postgres://postgres:password@localhost:5432");
    let db_name = Uuid::new_v4().to_string();

    let mut connection = PgConnection::connect(db_url.as_str())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, db_name))
        .await
        .expect("Failed to create database.");

    let db_url_with_db_name = format!("{}/{}", db_url, db_name);

    let figment = rocket::Config::figment().merge((
        "databases.newsletter",
        rocket_db_pools::Config {
            url: db_url_with_db_name,
            min_connections: None,
            max_connections: 1024,
            connect_timeout: 3,
            idle_timeout: None,
        },
    ));

    rocket::custom(figment)
        .attach(Newsletter::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .mount("/health_check", routes![health_check_route])
        .mount("/subscriptions", routes![subscribe])
}
