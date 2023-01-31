use clap::{CommandFactory, Parser};
use colored::Colorize;
use std::path::PathBuf;

mod cli;
mod document;
mod included;
mod ioutil;
mod log;
mod paths;
mod pdf;
mod themes;
mod watcher;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    if let Some(generator) = cli.generator {
        let mut cmd = cli::Cli::command();
        cli::print_completions(generator, &mut cmd);
        return Ok(());
    }

    if cli.list_themes {
        for theme in themes::available_themes()?.themes.into_iter() {
            println!("{}", theme.name);
        }

        return Ok(());
    }

    if cli.where_config {
        println!("{}", paths::dirs::config().display());

        return Ok(());
    }

    if let Some(path) = &cli.path {
        if path.is_dir() {
            die!("Path is a directory")
        }

        if !path.exists() {
            die!("No such file")
        }
    }

    let options = document::RenderOptions {
        theme: cli.get_theme()?,
        math: cli.all || cli.math,
        highlight: cli.all || cli.highlight,
        diagrams: cli.all || cli.diagrams,
        pdf: cli.pdf,
    };

    info!("Using theme {}", options.theme.name.cyan());
    if options.highlight {
        info!("Highlight.js syntax highlighting is enabled");
    }

    if options.math {
        info!("KaTeX math rendering is enabled");
    }

    if options.diagrams {
        info!("Mermaid diagrams rendering is enabled");
    }

    let out = {
        let auto_extension = if cli.pdf { "pdf" } else { "html" };

        if let Some(out) = &cli.out {
            out.clone()
        } else if let Some(path) = &cli.path {
            path.with_extension(auto_extension)
        } else {
            PathBuf::new()
                .with_file_name("out")
                .with_extension(auto_extension)
        }
    };

    if cli.watch {
        if cli.path.is_none() {
            die!("watcher needs a file to watch");
        }

        watcher::watch(&cli.path.unwrap(), &out, &options)?;

        return Ok(());
    }

    let md = cli.get_markdown()?;
    let doc = document::Document::new(&md);
    let buffer = doc.render(&options)?;

    if cli.stdout {
        let string = String::from_utf8(buffer).unwrap();
        println!("{}", string);
    } else {
        std::fs::write(&out, &buffer)?;
        info!(
            "wrote {} to {}",
            humansize::format_size(buffer.len(), humansize::DECIMAL),
            &out.display().to_string().cyan(),
        );

        if cli.open {
            open::that(&out)?;
        }
    }

    Ok(())
}

fn main() {
    let execution_start = std::time::Instant::now();

    match run() {
        Ok(_) => {
            let execution_duration = execution_start.elapsed();

            success!(
                "took {}",
                format!("{}ms", execution_duration.as_millis()).yellow(),
            )
        }
        Err(e) => die!("{}", e.to_string()),
    }
}
