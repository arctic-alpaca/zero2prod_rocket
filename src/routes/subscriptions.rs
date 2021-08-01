use crate::startup::Newsletter;
use chrono::Utc;
use rocket::form::FromForm;
use rocket::form::{Form, Strict};
use rocket::http::Status;
use rocket::post;
use rocket_db_pools::Connection;
use uuid::Uuid;

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
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form_data.email,
        form_data.name,
        Utc::now()
    )
    .execute(&mut *db)
    .await;

    Status::Ok
}
