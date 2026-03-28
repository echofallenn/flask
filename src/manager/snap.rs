use anyhow::Result;
use async_trait::async_trait;
use crate::config::Config;
use crate::manager::PackageManager;

pub struct Snap { _cfg: Config }

impl Snap {
    pub fn new(cfg: Config) -> Self { Snap { _cfg: cfg } }
}

#[async_trait]
impl PackageManager for Snap {
    fn name(&self) -> &'static str { "snap" }

    async fn install(&self, packages: &[String]) -> Result<()> {
        let args: Vec<&str> = std::iter::once("install")
            .chain(packages.iter().map(String::as_str))
            .collect();
        self.run_cmd("snap", &args).await
    }

    async fn remove(&self, packages: &[String]) -> Result<()> {
        let args: Vec<&str> = std::iter::once("remove")
            .chain(packages.iter().map(String::as_str))
            .collect();
        self.run_cmd("snap", &args).await
    }

    async fn search(&self, query: &str) -> Result<()> {
        self.run_cmd("snap", &["find", query]).await
    }

    async fn update(&self, _packages: &[String]) -> Result<()> {
        self.run_cmd("snap", &["refresh"]).await
    }

    async fn list(&self) -> Result<()> {
        self.run_cmd("snap", &["list"]).await
    }

    async fn info(&self, package: &str) -> Result<()> {
        self.run_cmd("snap", &["info", package]).await
    }
}
