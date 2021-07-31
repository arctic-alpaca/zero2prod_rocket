use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use zero2prod_rocket::run;

// Random port is not needed, rocket creates a local instance without binding to a port.
// Tests are executed by requests being passed to rocket without networking (skipping hyper)

#[test]
fn health_check_works() {
    let client = Client::tracked(run()).expect("Could not start Rocket instance.");
    let response = client.get("/health_check").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), None);
}

#[test]
fn subscribe_returns_a_200_for_valid_form_data() {
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let client = Client::tracked(run()).expect("Could not start Rocket instance.");
    let response = client
        .post("/subscriptions")
        .header(ContentType::Form)
        .body(body)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn subscribe_returns_a_400_when_data_is_missing() {
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
