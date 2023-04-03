use self::db::Database;
use axum::{
    extract::State,
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    routing::{get, post},
    Json, Router, Server,
};
use chrono::{DateTime, Utc};
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::{
    convert::Infallible,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::sync::broadcast::{channel, Sender};

//

mod db;

//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    id: String,
    date: DateTime<Utc>,
    content: String,
}

#[derive(Clone)]
struct AppState {
    db: Database,
    tx: Sender<Message>,
}

//

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .route("/", get(root))
        .route("/", post(set_root))
        .route("/sse", get(listen))
        .with_state(AppState {
            db: Database::new().await?,
            tx: channel(1).0,
        });

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);

    let server = Server::bind(&addr).serve(router.into_make_service());
    println!("Listening on http://{}/", server.local_addr());

    server.await?;

    Ok(())
}

async fn set_root(
    State(state): State<AppState>,
    Json(val): Json<String>,
) -> (StatusCode, Json<()>) {
    let msg = state.db.send_msg(&val).await.unwrap();
    state.tx.send(msg).unwrap();

    (StatusCode::CREATED, Json(()))
}

async fn root(State(state): State<AppState>) -> Json<Vec<Message>> {
    Json(state.db.get_msg().await.unwrap())
}

async fn listen(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut rx = state.tx.subscribe();

    let stream = async_stream::stream! {
        while let Ok(v) = rx.recv().await {
            yield Ok(Event::default().json_data(v).unwrap())
        }
    };

    // axum::response::sse;

    Sse::new(stream).keep_alive(KeepAlive::default())
}
