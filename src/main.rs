use clap::{ArgGroup, Parser};
use colored::Colorize;
use notify::Watcher;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

mod document;
mod included;
mod log;
mod paths;
mod themes;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("input")
        .args(&["path", "string", "stdin"])
        .conflicts_with("info")
))]
struct Cli {
    #[arg(short, long, help = "Theme to use")]
    theme: Option<String>,

    #[arg(short, long, help = "Read input from stdin")]
    stdin: bool,

    #[arg(help = "Read input from file")]
    path: Option<PathBuf>,

    #[arg(long, help = "Read input from string")]
    string: Option<String>,

    #[arg(short, long, group = "info", help = "List available themes")]
    list_themes: bool,

    #[arg(long, group = "info", help = "Print config path")]
    where_config: bool,

    #[arg(short, long, help = "Output file")]
    out: Option<PathBuf>,

    #[arg(short = 'H', long, help = "Enable syntax highligting")]
    syntax_highlighting: bool,

    #[arg(short, long, help = "Enable file watcher")]
    watch: bool,
}

impl Cli {
    pub fn get_markdown(&self) -> io::Result<String> {
        if let Some(path) = &self.path {
            return read_path(&path);
        }

        if let Some(string) = &self.string {
            return Ok(string.clone());
        }

        if self.stdin {
            return read_stdin();
        }

        error!("no input is given, see --help");
        std::process::exit(1);
    }

    pub fn get_theme(&self) -> Result<themes::Theme, Box<dyn std::error::Error>> {
        match &self.theme {
            Some(name) => {
                let available = themes::available_themes()?;

                match available.by_name(name) {
                    Some(theme) => Ok(theme),
                    None => {
                        error!("unknown theme '{}'", name);
                        std::process::exit(1);
                    }
                }
            }
            None => Ok(themes::Theme::default()),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

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

    let options = document::RenderOptions {
        theme: cli.get_theme()?,
        highlight: cli.syntax_highlighting,
    };

    if cli.watch {
        if cli.path.is_none() {
            error!("watcher needs a file to watch");
            std::process::exit(1);
        }

        watch(&cli.path.unwrap(), cli.out.as_ref(), &options)?;

        return Ok(());
    }

    let contents = cli.get_markdown()?;
    let doc = document::Document::new(&contents);
    let html = doc.render(&options)?;

    if let Some(out_path) = &cli.out {
        std::fs::write(out_path, html)?;
    } else {
        println!("{}", html);
    }

    Ok(())
}

fn read_stdin() -> io::Result<String> {
    let mut buffer = Vec::new();
    let mut stdin = io::stdin();
    stdin.read_to_end(&mut buffer)?;

    Ok(String::from_utf8(buffer).unwrap())
}

fn read_path(path: &PathBuf) -> io::Result<String> {
    let mut buffer = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn watch(
    path: &PathBuf,
    output: Option<&PathBuf>,
    options: &document::RenderOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.clone();
    let output_path = match output {
        Some(path) => path.clone(),
        None => {
            let mut path = path.clone();
            path.set_extension("html");
            path
        }
    };

    info!(
        "watching {} -> {}",
        path.display().to_string().cyan(),
        output_path.display().to_string().cyan()
    );

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::RecommendedWatcher::new(tx, notify::Config::default())?;

    watcher.watch(path.as_path(), notify::RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind.is_modify() {
                    let read_res = read_path(&path);

                    if let Ok(contents) = read_res {
                        let execution_start = std::time::Instant::now();
                        let doc = document::Document::new(&contents);
                        let html = doc.render(&options)?;
                        let execution_duration = execution_start.elapsed();

                        match std::fs::write(&output_path, html.as_str()) {
                            Ok(_) => info!(
                                "{} updated, wrote {}B to {} in {}ms",
                                path.display(),
                                html.len(),
                                output_path.display(),
                                execution_duration.as_millis()
                            ),
                            Err(e) => error!("{}", e.to_string()),
                        }
                    }
                }
            }
            Err(e) => error!("{}", e.to_string()),
        }
    }

    Ok(())
}
