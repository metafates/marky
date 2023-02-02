use std::{cell::RefCell, net::SocketAddr, path::PathBuf};

use crate::document;
use crate::{info, service, warn};
use axum::{routing::get, Extension, Router};
use colored::Colorize;
use tokio::sync::{
    oneshot,
    watch::{self, Sender},
};

#[derive(Clone)]
pub struct Config {
    pub root_dir: PathBuf,
    pub render_options: document::RenderOptions,
}

/// Code is taken from the https://github.com/euclio/aurelius/

/// Markdown preview server.
///
/// Listens for HTTP connections and serves a page containing a live markdown preview. The page
/// contains JavaScript to open a websocket connection back to the server for rendering updates.
#[derive(Debug)]
pub struct Server {
    pub addr: SocketAddr,
    output: RefCell<String>,
    tx: Sender<String>,
    _shutdown_tx: oneshot::Sender<()>,
}

impl Server {
    pub fn bind(addr: &SocketAddr, config: Config) -> Self {
        let (tx, rx) = watch::channel(String::new());
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        let app = Router::new()
            .route("/", get(service::websocket_handler))
            .fallback_service(get(service::serve_static_file))
            .layer(Extension(rx))
            .layer(Extension(config));

        let http_server = axum::Server::bind(addr).serve(app.into_make_service());
        let addr = http_server.local_addr();

        info!("Listening on {}", addr);
        info!("Opening in browser");

        if let Err(e) = open::that(format!("http://{}", addr)) {
            warn!("Failed to open the page: {}", e);
        }

        let http_server = http_server.with_graceful_shutdown(async move {
            let _ = shutdown_rx.await;
        });

        tokio::spawn(http_server);

        Server {
            addr,
            output: RefCell::new(String::new()),
            tx,
            _shutdown_tx: shutdown_tx,
        }
    }

    pub async fn send(&self, document: &crate::document::Document) {
        self.output
            .replace(self.tx.send_replace(document.render_body()));
    }
}
