use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: General,
    pub backends: Backends,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct General {
    pub default_backend: String,
    pub confirm: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Backends {
    pub snap: BackendConfig,
    pub flatpak: BackendConfig,
    pub pacman: BackendConfig,
    pub aur: AurConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackendConfig {
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AurConfig {
    pub enabled: bool,
    pub helper: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: General {
                default_backend: "pacman".into(),
                confirm: true,
            },
            backends: Backends {
                snap:    BackendConfig { enabled: true },
                flatpak: BackendConfig { enabled: true },
                pacman:  BackendConfig { enabled: true },
                aur: AurConfig {
                    enabled: true,
                    helper: "yay".into(),
                },
            },
        }
    }
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("flask")
        .join("flask.toml")
}

pub fn load() -> Result<Config> {
    let path = config_path();

    if !path.exists() {
        create_default(&path)?;
        return Ok(Config::default());
    }

    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    toml::from_str(&raw)
        .with_context(|| format!("failed to parse {}", path.display()))
}

fn create_default(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }

    let default = Config::default();
    let contents = toml::to_string_pretty(&default)
        .context("failed to serialize default config")?;

    fs::write(path, contents)
        .with_context(|| format!("failed to write {}", path.display()))?;

    eprintln!("created config at {}", path.display());
    Ok(())
}
