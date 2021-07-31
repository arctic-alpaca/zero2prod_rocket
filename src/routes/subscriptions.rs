use crate::startup::Newsletter;

use rocket::form::FromForm;
use rocket::form::{Form, Strict};
use rocket::http::Status;
use rocket::post;
use rocket_db_pools::Connection;

#[derive(FromForm)]
pub struct FormData {
    email: String,
    name: String,
}

#[post("/", data = "<form_data>")]
pub async fn subscribe(
    mut db: Connection<Newsletter>,
    form_data: Form<Strict<FormData>>,
) -> Status {
    Status::Ok
}
