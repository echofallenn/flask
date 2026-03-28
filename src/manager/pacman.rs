use anyhow::Result;
use async_trait::async_trait;
use crate::config::Config;
use crate::manager::PackageManager;

pub struct Pacman { _cfg: Config }

impl Pacman {
    pub fn new(cfg: Config) -> Self { Pacman { _cfg: cfg } }
}

#[async_trait]
impl PackageManager for Pacman {
    fn name(&self) -> &'static str { "pacman" }

    async fn install(&self, packages: &[String]) -> Result<()> {
        let mut args = vec!["pacman", "-S", "--noconfirm"];
        args.extend(packages.iter().map(String::as_str));
        self.run_cmd("sudo", &args).await
    }

    async fn remove(&self, packages: &[String]) -> Result<()> {
        let mut args = vec!["pacman", "-Rns", "--noconfirm"];
        args.extend(packages.iter().map(String::as_str));
        self.run_cmd("sudo", &args).await
    }

    async fn search(&self, query: &str) -> Result<()> {
        self.run_cmd("pacman", &["-Ss", query]).await
    }

    async fn update(&self, _packages: &[String]) -> Result<()> {
        self.run_cmd("sudo", &["pacman", "-Syu", "--noconfirm"]).await
    }

    async fn list(&self) -> Result<()> {
        self.run_cmd("pacman", &["-Q"]).await
    }

    async fn info(&self, package: &str) -> Result<()> {
        self.run_cmd("pacman", &["-Si", package]).await
    }
}
