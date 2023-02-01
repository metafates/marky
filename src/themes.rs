use crate::warn;
use crate::{included::VENDOR_DIR, paths};
use anyhow::{Error, Result};
use colored::Colorize;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(serde::Deserialize, Clone)]
pub struct Theme {
    pub name: String,

    path: Option<PathBuf>,
    inline: Option<String>,
}

impl Theme {
    pub fn resolve(&self) -> Result<String> {
        let resolved = {
            if self.inline.is_some() {
                Some(self.resolve_inline()?)
            } else if self.path.is_some() {
                Some(self.resolve_path()?)
            } else {
                None
            }
        };

        match resolved {
            Some(css) => match minifier::css::minify(css.as_str()) {
                Ok(minfied) => Ok(minfied.to_string()),
                Err(error) => Err(Error::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    error,
                ))),
            },
            None => Err(Error::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "theme source is not specified",
            ))),
        }
    }

    fn resolve_inline(&self) -> std::io::Result<String> {
        assert!(self.inline.is_some());

        let inline = self.inline.as_ref().unwrap().to_string();
        Ok(inline)
    }

    fn resolve_path(&self) -> std::io::Result<String> {
        assert!(self.path.is_some());

        let path = {
            let path = self.path.as_ref().unwrap();

            if path.is_relative() {
                paths::dirs::config().join(path)
            } else {
                path.clone()
            }
        };

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Themes::default().by_name("sakura").unwrap()
    }
}

#[derive(serde::Deserialize)]
pub struct Themes {
    pub themes: Vec<Theme>,
}

impl Themes {
    pub fn by_name(&self, name: &str) -> Option<Theme> {
        self.themes.iter().find(|theme| theme.name == name).cloned()
    }

    pub fn closest_match(&self, name: &str) -> Option<Theme> {
        use levenshtein::levenshtein;

        self.themes
            .iter()
            .min_by(|a, b| {
                levenshtein(name, a.name.as_str()).cmp(&levenshtein(name, b.name.as_str()))
            })
            .cloned()
    }
}

impl Default for Themes {
    fn default() -> Self {
        let themes: Vec<Theme> = VENDOR_DIR
            .get_dir("themes")
            .expect("themes directory in vendor/ must be present")
            .entries()
            .into_iter()
            .filter_map(|entry| entry.as_file())
            .filter_map(|file| {
                let path = file.path();

                if path.extension().map(|ext| ext != "css").unwrap_or(true) {
                    return None;
                }

                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                match std::str::from_utf8(file.contents()) {
                    Ok(contents) => Some((name, contents)),
                    Err(e) => {
                        warn!("can't parse theme {}: {}", name.cyan(), e);
                        None
                    }
                }
            })
            .map(|(name, contents)| Theme {
                name,
                inline: Some(contents.to_string()),
                path: None,
            })
            .collect();

        Themes { themes }
    }
}

pub fn available_themes() -> Result<Themes> {
    let mut default = Themes::default();

    let themes_path = paths::files::themes();
    if !themes_path.exists() {
        return Ok(default);
    }

    let mut file = File::open(themes_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut custom: Themes = toml::from_str(contents.as_str())?;

    default.themes.append(&mut custom.themes);

    Ok(default)
}
