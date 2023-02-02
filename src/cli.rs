use crate::{die, error, ioutil, note, themes};
use clap::{ArgGroup, Command, Parser, ValueHint};
use clap_complete::{Generator, Shell};
use colored::Colorize;
use std::{io, path::PathBuf};

#[derive(Parser)]
#[command(name = "marky", author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("input")
        .args(&["path", "string"])
        .conflicts_with("info")
))]
#[clap(group(
    ArgGroup::new("output")
        .args(&["out", "stdout"])
        .conflicts_with("info")
))]
#[clap(group(
    ArgGroup::new("watchers")
        .args(&["watch", "live"])
        .conflicts_with("info")
))]
pub struct Cli {
    #[arg(long = "completion", value_enum)]
    pub generator: Option<Shell>,

    #[arg(short, long, help = "Theme to use")]
    pub theme: Option<String>,

    #[arg(help = "Read input from file", value_hint = ValueHint::FilePath)]
    pub path: Option<PathBuf>,

    #[arg(long, help = "Read input from string")]
    pub string: Option<String>,

    #[arg(long, group = "info", help = "List available themes")]
    pub themes: bool,

    #[arg(long, group = "info", help = "Print config path")]
    pub where_config: bool,

    #[arg(short, long, help = "Output file", value_hint = ValueHint::FilePath)]
    pub out: Option<PathBuf>,

    #[arg(long, help = "Output to stdout")]
    pub stdout: bool,

    #[arg(
        short = 'H',
        long,
        help = "Enable syntax highligting with highlight.js"
    )]
    pub highlight: bool,

    #[arg(short = 'M', long, help = "Enable math rendering with KaTeX")]
    pub math: bool,

    #[arg(short = 'D', long, help = "Enable UML diagrams rendering with Mermaid")]
    pub diagrams: bool,

    #[arg(short = 'A', long, help = "Enable all extra renderers")]
    pub all: bool,

    #[arg(short, long, help = "Recompile file on save")]
    pub watch: bool,

    #[arg(short, long, help = "Live preview in the browser")]
    pub live: bool,

    #[arg(long, default_value = "8080", help = "Port of the live server")]
    pub port: u16,

    #[arg(short = 'O', long, help = "Open output file in the default app")]
    pub open: bool,

    #[arg(
        short,
        long,
        help = "Saves document as PDF, will auto-download headless-chrome"
    )]
    pub pdf: bool,
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout())
}

impl Cli {
    pub fn get_markdown(&self) -> io::Result<String> {
        if atty::isnt(atty::Stream::Stdin) {
            return ioutil::read_stdin();
        }

        if let Some(path) = &self.path {
            return ioutil::read_path(&path);
        }

        if let Some(string) = &self.string {
            return Ok(string.clone());
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
