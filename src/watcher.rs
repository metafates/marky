use anyhow::Result;
use colored::Colorize;
use notify::Watcher;

use crate::{document, error, info, ioutil};
use std::{
    io, net,
    path::{Path, PathBuf},
};

fn recompile(path: &PathBuf, options: &document::RenderOptions) -> io::Result<document::Document> {
    match ioutil::read_path(path) {
        Ok(contents) => Ok(document::Document {
            text: contents,
            options: options.clone(),
        }),
        Err(e) => Err(e),
    }
}

macro_rules! watch {
    ($path: ident, $options:ident, $on_update:ident$(.$field:ident)*$( $arg:ident)*) => {{
        info!("waiting for changes on {}", $path.display().to_string().cyan());

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::RecommendedWatcher::new(tx, notify::Config::default())?;

        watcher.watch($path.as_path(), notify::RecursiveMode::NonRecursive)?;

        if let Ok(compiled) = recompile($path, $options) {
            $on_update$(.$field)*($($arg,)* &compiled).await;
        }

        for res in rx {
            match res {
                Ok(event) => {
                    if event.kind.is_modify() {
                        match recompile($path, $options) {
                            Ok(compiled) => {
                                $on_update$(.$field)*($($arg,)* &compiled).await;
                                info!("updated")
                            },
                            Err(e) => error!("compilation failed: {}", e)
                        }
                    }
                }
                Err(e) => error!("{}", e.to_string()),
            }
        }

        Ok(())
    }};
}

pub async fn watch_live(
    path: &PathBuf,
    options: &document::RenderOptions,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = net::SocketAddr::V4(net::SocketAddrV4::new(
        net::Ipv4Addr::new(127, 0, 0, 1),
        port,
    ));

    let config = crate::server::Config {
        root_dir: path
            .clone()
            .parent()
            .unwrap_or(Path::new("."))
            .to_path_buf(),
        render_options: options.clone(),
    };

    let server = crate::server::Server::bind(&addr, config);

    watch!(path, options, server.send)
}

pub async fn watch_file(
    path: &PathBuf,
    output: &PathBuf,
    options: &document::RenderOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    watch!(path, options, write_to_file output)
}

async fn write_to_file(path: &PathBuf, document: &document::Document) {
    match document.render() {
        Ok(rendered) => {
            if let Err(e) = std::fs::write(path, rendered) {
                error!("{}", e);
            }
        }
        Err(e) => error!("{}", e),
    }
}
