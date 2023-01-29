use std::path::PathBuf;

pub mod dirs {
    use super::PathBuf;

    pub fn config() -> PathBuf {
        let config_dir = dirs::config_dir().unwrap_or(".".into());

        config_dir.join("marky")
    }
}

pub mod files {
    use super::PathBuf;

    pub fn themes() -> PathBuf {
        super::dirs::config().join("themes.toml")
    }
}
