use rocket::get;
use rocket::http::Status;

#[get("/")]
pub async fn health_check_route() -> Status {
    Status::Ok
}
