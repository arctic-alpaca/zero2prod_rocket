use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket_db_pools::Database;
use zero2prod_rocket::startup::{run, run_with_random_database_name, Newsletter};

// Random port is not needed, rocket creates a local instance without binding to a port.
// Tests are executed by requests being passed to rocket without networking (skipping hyper)

#[test]
fn health_check_works() {
    let client = Client::tracked(run()).expect("Could not start Rocket instance.");
    let response = client.get("/health_check").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), None);
}

#[rocket::async_test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    use rocket::local::asynchronous::Client;

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let client = Client::tracked(run_with_random_database_name().await)
        .await
        .expect("Could not start Rocket instance.");

    let response = client
        .post("/subscriptions")
        .header(ContentType::Form)
        .body(body)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let db = Newsletter::fetch(client.rocket()).unwrap();
    let mut con = db.acquire().await.unwrap();

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut con)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[test]
fn subscribe_returns_a_422_when_data_is_missing() {
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    let client = Client::tracked(run()).expect("Could not start Rocket instance.");

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post("/subscriptions")
            .header(ContentType::Form)
            .body(invalid_body)
            .dispatch();
        assert_eq!(
            response.status(),
            Status::UnprocessableEntity,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
