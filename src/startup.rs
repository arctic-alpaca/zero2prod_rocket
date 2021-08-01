use rocket::routes;
use rocket::{Build, Rocket};
use rocket_db_pools::{sqlx, Database};

use crate::routes::{health_check_route, subscribe};

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
