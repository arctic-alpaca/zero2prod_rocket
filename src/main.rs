#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use zero2prod_rocket::startup::run;

#[launch]
pub async fn rocket() -> Rocket<Build> {
    run()
}
