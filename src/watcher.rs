use colored::Colorize;
use notify::Watcher;

use crate::{document, error, info, ioutil, success};
use std::path::PathBuf;

pub fn watch(
    path: &PathBuf,
    output: &PathBuf,
    options: &document::RenderOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.clone();

    info!(
        "watching {} --> {}",
        path.display().to_string().cyan(),
        output.display().to_string().cyan()
    );

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::RecommendedWatcher::new(tx, notify::Config::default())?;

    watcher.watch(path.as_path(), notify::RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                use notify::{event::DataChange, event::ModifyKind, EventKind};
                if let EventKind::Modify(ModifyKind::Data(DataChange::Content)) = event.kind {
                    let read_res = ioutil::read_path(&path);

                    if let Ok(contents) = read_res {
                        let execution_start = std::time::Instant::now();
                        let doc = document::Document::new(&contents);
                        let buffer = doc.render(&options)?;
                        let execution_duration = execution_start.elapsed();

                        match std::fs::write(&output, &buffer) {
                            Ok(_) => success!(
                                "{} updated, wrote {} to {} in {}",
                                path.display().to_string().cyan(),
                                humansize::format_size(buffer.len(), humansize::DECIMAL),
                                output.display().to_string().cyan(),
                                format!("{}ms", execution_duration.as_millis()).yellow(),
                            ),
                            Err(e) => error!("{}", e.to_string()),
                        }
                    }
                };
            }
            Err(e) => error!("{}", e.to_string()),
        }
    }

    Ok(())
}
