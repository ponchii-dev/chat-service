use tokio::net::TcpListener;
mod route;
mod telemetry;
mod handlers;
mod ws;
mod models;
mod broker;

use broker::Broker;

/// Shared app state
#[derive(Clone)]
pub struct AppState {
    pub broker: Broker,
}

#[tokio::main]
async fn main() {
    telemetry::init_tracing();

    // In-memory broadcast hub
    let state = AppState {
        broker: Broker::new(),
    };

    let app = route::app(state);

    let addr = "0.0.0.0:3001";
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    println!("Listening on http://{addr}");

    axum::serve(listener, app)
        .await
        .expect("server crashed");
}
