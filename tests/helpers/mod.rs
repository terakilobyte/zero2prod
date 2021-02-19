use std::net::TcpListener;

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("could not bind to a port");
    let addr = listener.local_addr().expect("foo").to_string();
    let server = zero2prod::run(listener).expect("Failed to spawn our app.");

    let _ = tokio::spawn(server);
    addr
}
