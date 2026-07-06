use std::sync::Arc;

use axum::Router;
use cgp_example_transfer::contexts::MockApp;
use cgp_example_transfer::providers::CanAddMainApiRoutes;
use tokio::net::TcpListener;

// Entry point: build the wired `MockApp`, mount its endpoints on an Axum router (the routes
// come from the `add_main_api_routes` capability wired in the library), and serve.
#[tokio::main]
async fn main() {
    let app = Arc::new(MockApp::new_with_dummy_data());

    let router = <Router<Arc<MockApp>>>::new()
        .add_main_api_routes()
        .with_state(app);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
