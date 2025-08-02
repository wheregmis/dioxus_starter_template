use dioxus::server::{DioxusRouterExt, ServeConfig};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration};
use ui::App;

pub async fn run_server() {
    let address = dioxus::cli_config::fullstack_address_or_localhost();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));

    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::new().expect("Failed to load index"), App)
        .layer(session_layer);

    let router = router.into_make_service();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[tokio::main]
async fn main() {
    run_server().await;
}
