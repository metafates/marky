use axum::{
    body::Body,
    extract::{
        ws::{Message as AxumMessage, WebSocket, WebSocketUpgrade},
        Extension,
    },
    http::{Request, StatusCode},
    response::{Html, IntoResponse},
};
use tokio::sync::watch::Receiver;
use tower::util::ServiceExt;
use tower_http::services::ServeDir;

pub async fn websocket_handler(
    ws: Option<WebSocketUpgrade>,
    Extension(config): Extension<crate::server::Config>,
    Extension(html_rx): Extension<Receiver<String>>,
) -> impl IntoResponse {
    if let Some(ws) = ws {
        return ws.on_upgrade(|ws| async { handle_websocket(ws, html_rx).await });
    }

    let doc = crate::document::Document {
        text: "😴 Waiting for changes".into(),
        options: config.render_options,
    };

    let buffer = doc.render().expect("Document with empty text must render");
    let html = String::from_utf8(buffer).expect("Must be a valid utf8");
    (StatusCode::OK, Html(html)).into_response()
}

async fn handle_websocket(mut socket: WebSocket, mut html_rx: Receiver<String>) {
    while html_rx.changed().await.is_ok() {
        let html = html_rx.borrow().clone();
        socket.send(AxumMessage::Text(html)).await.unwrap();
    }

    let _ = socket.send(AxumMessage::Close(None)).await;
}

pub async fn serve_static_file(
    Extension(config): Extension<crate::server::Config>,
    req: Request<Body>,
) -> impl IntoResponse {
    let service = ServeDir::new(config.root_dir);

    service
        .oneshot(req)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))
}
