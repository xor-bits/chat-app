use axum::{
    extract::State,
    http::StatusCode,
    response::Sse,
    routing::{get, post},
    Json, Router, Server,
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::sync::RwLock;
// use self::db::Database;

//

// mod db;

// #[derive(Clone)]
// struct AppState {
//     db: Database,
// }

//

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root))
        .route("/", post(set_root))
        .with_state(Arc::new(RwLock::new("Hello, world!".into())));

    let server = Server::bind(&SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 0))
        .serve(router.into_make_service());
    println!("Listening on http://{}/", server.local_addr());

    server.await.unwrap();
}

async fn set_root(
    State(state): State<Arc<RwLock<String>>>,
    Json(val): Json<String>,
) -> (StatusCode, Json<()>) {
    *state.write().await = val;

    (StatusCode::CREATED, Json(()))
}

async fn root(State(state): State<Arc<RwLock<String>>>) -> String {
    state.read().await.clone()
}

/* async fn listen() -> Sse {
    axum::response::sse;
} */
