use rocket::form::FromForm;
use rocket::form::{Form, Strict};
use rocket::get;
use rocket::http::Status;
use rocket::post;
use rocket::routes;
use rocket::{Build, Rocket};
use rocket_db_pools::Connection;
use rocket_db_pools::Database;

use crate::routes::health_check;
use crate::routes::subscribe;

#[derive(Database)]
#[database("newsletter")]
pub(crate) struct Newsletter(rocket_db_pools::sqlx::PgPool);

pub fn run() -> Rocket<Build> {
    //let figment = rocket::Config::figment().merge(("port", 0));
    //rocket::custom(figment).mount("/health_check", routes![health_check])
    rocket::build()
        .attach(Newsletter::init())
        .mount("/health_check", routes![health_check])
        .mount("/subscriptions", routes![subscribe])
}
