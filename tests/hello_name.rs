mod helpers;
use helpers::spawn_app;
#[actix_rt::test]
async fn health_check_works() {
    let addr = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/{}", &addr, "hello/foo"))
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
