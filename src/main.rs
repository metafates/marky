use clap::{ArgGroup, Command, CommandFactory, Parser, ValueHint};
use clap_complete::{Generator, Shell};
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
#[command(name = "marky", author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("input")
        .args(&["path", "string", "stdin"])
        .conflicts_with("info")
))]
#[clap(group(
    ArgGroup::new("output")
        .args(&["out", "stdout"])
        .conflicts_with("info")
))]
struct Cli {
    #[arg(long = "completion", value_enum)]
    generator: Option<Shell>,

    #[arg(short, long, help = "Theme to use")]
    theme: Option<String>,

    #[arg(long, help = "Read input from stdin")]
    stdin: bool,

    #[arg(help = "Read input from file", value_hint = ValueHint::FilePath)]
    path: Option<PathBuf>,

    #[arg(long, help = "Read input from string")]
    string: Option<String>,

    #[arg(short, long, group = "info", help = "List available themes")]
    list_themes: bool,

    #[arg(long, group = "info", help = "Print config path")]
    where_config: bool,

    #[arg(short, long, help = "Output file", value_hint = ValueHint::FilePath)]
    out: Option<PathBuf>,

    #[arg(long, help = "Output to stdout")]
    stdout: bool,

    #[arg(short = 'H', long, help = "Enable syntax highligting")]
    highlight: bool,

    #[arg(short = 'M', long, help = "Enable math rendering (LaTeX)")]
    math: bool,

    #[arg(short, long, help = "Enable file watcher")]
    watch: bool,

    #[arg(short = 'O', long, help = "Open output file in the default app")]
    open: bool,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout())
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

        die!("no input is given, see {}", "--help".yellow());
    }

    pub fn get_theme(&self) -> Result<themes::Theme, Box<dyn std::error::Error>> {
        match &self.theme {
            Some(name) => {
                let available = themes::available_themes()?;

                match available.by_name(name) {
                    Some(theme) => Ok(theme),
                    None => {
                        error!("unknown theme {}", name.cyan());

                        if let Some(closest) = available.closest_match(name) {
                            note!("theme {} exists", closest.name.cyan());
                        }

                        die!();
                    }
                }
            }
            None => Ok(themes::Theme::default()),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        print_completions(generator, &mut cmd);
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

    let options = document::RenderOptions {
        theme: cli.get_theme()?,
        math: cli.math,
        highlight: cli.highlight,
    };

    info!("using theme {}", options.theme.name.cyan());
    if options.highlight {
        info!("code syntax highlighting is enabled");
    }

    let out = {
        if let Some(out) = &cli.out {
            out.clone()
        } else if let Some(path) = &cli.path {
            path.with_extension("html")
        } else {
            PathBuf::new().with_file_name("out").with_extension("html")
        }
    };

    if cli.watch {
        if cli.path.is_none() {
            die!("watcher needs a file to watch");
        }

        watch(&cli.path.unwrap(), &out, &options)?;

        return Ok(());
    }

    let execution_start = std::time::Instant::now();
    let contents = cli.get_markdown()?;
    let doc = document::Document::new(&contents);
    let html = doc.render(&options)?;
    let execution_duration = execution_start.elapsed();

    let formatted_millis = format!("{}ms", execution_duration.as_millis()).yellow();

    if cli.stdout {
        println!("{}", html);
        info!("took {}", formatted_millis);
    } else {
        std::fs::write(&out, &html)?;
        info!(
            "wrote {}B to {} in {}",
            html.len(),
            &out.display(),
            formatted_millis,
        );

        if cli.open {
            open::that(&out)?;
        }
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
    output: &PathBuf,
    options: &document::RenderOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.clone();

    info!(
        "watching {} -> {}",
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
                match event.kind {
                    EventKind::Modify(ModifyKind::Data(DataChange::Content)) => {
                        let read_res = read_path(&path);

                        if let Ok(contents) = read_res {
                            let execution_start = std::time::Instant::now();
                            let doc = document::Document::new(&contents);
                            let html = doc.render(&options)?;
                            let execution_duration = execution_start.elapsed();

                            match std::fs::write(&output, html.as_str()) {
                                Ok(_) => info!(
                                    "{} updated, wrote {}B to {} in {}",
                                    path.display(),
                                    html.len(),
                                    output.display(),
                                    format!("{}ms", execution_duration.as_millis()).yellow(),
                                ),
                                Err(e) => error!("{}", e.to_string()),
                            }
                        }
                    }
                    _ => {}
                };
            }
            Err(e) => error!("{}", e.to_string()),
        }
    }

    Ok(())
}
