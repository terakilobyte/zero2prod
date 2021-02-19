#[actix_rt::test]
async fn health_check_works() {
    spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8080/hello/foo")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let text = response
        .text()
        .await
        .expect("couldn't handle response text");
    assert_eq!(text, "Hello, foo");
}

async fn spawn_app() {
    let server = zero2prod::run().expect("Failed to spawn our app.");

    let _ = tokio::spawn(server);
}
