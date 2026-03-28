use anyhow::Result;
use async_trait::async_trait;
use crate::config::Config;
use crate::manager::PackageManager;

pub struct Flatpak { _cfg: Config }

impl Flatpak {
    pub fn new(cfg: Config) -> Self { Flatpak { _cfg: cfg } }
}

#[async_trait]
impl PackageManager for Flatpak {
    fn name(&self) -> &'static str { "flatpak" }

    async fn install(&self, packages: &[String]) -> Result<()> {
        let args: Vec<&str> = ["install", "--assumeyes"]
            .into_iter()
            .chain(packages.iter().map(String::as_str))
            .collect();
        self.run_cmd("flatpak", &args).await
    }

    async fn remove(&self, packages: &[String]) -> Result<()> {
        let args: Vec<&str> = ["uninstall", "--assumeyes"]
            .into_iter()
            .chain(packages.iter().map(String::as_str))
            .collect();
        self.run_cmd("flatpak", &args).await
    }

    async fn search(&self, query: &str) -> Result<()> {
        self.run_cmd("flatpak", &["search", query]).await
    }

    async fn update(&self, _packages: &[String]) -> Result<()> {
        self.run_cmd("flatpak", &["update", "--assumeyes"]).await
    }

    async fn list(&self) -> Result<()> {
        self.run_cmd("flatpak", &["list"]).await
    }

    async fn info(&self, package: &str) -> Result<()> {
        self.run_cmd("flatpak", &["info", package]).await
    }
}
