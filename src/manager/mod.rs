pub mod aur;
pub mod flatpak;
pub mod pacman;
pub mod snap;

use anyhow::{bail, Result};
use async_trait::async_trait;
use crate::config::Config;

#[async_trait]
pub trait PackageManager: Send + Sync {
    fn name(&self) -> &'static str;

    async fn install(&self, packages: &[String]) -> Result<()>;
    async fn remove(&self, packages: &[String]) -> Result<()>;
    async fn search(&self, query: &str) -> Result<()>;
    async fn update(&self, packages: &[String]) -> Result<()>;
    async fn list(&self) -> Result<()>;
    async fn info(&self, package: &str) -> Result<()>;

    async fn run_cmd(&self, cmd: &str, args: &[&str]) -> Result<()> {
        use tokio::process::Command;
        use anyhow::Context;

        let status = Command::new(cmd)
            .args(args)
            .status()
            .await
            .with_context(|| format!("failed to run `{cmd}`"))?;

        if !status.success() {
            bail!("`{cmd}` exited with status {status}");
        }
        Ok(())
    }
}

pub fn snap(cfg: Config)    -> Box<dyn PackageManager> { Box::new(snap::Snap::new(cfg)) }
pub fn flatpak(cfg: Config) -> Box<dyn PackageManager> { Box::new(flatpak::Flatpak::new(cfg)) }
pub fn pacman(cfg: Config)  -> Box<dyn PackageManager> { Box::new(pacman::Pacman::new(cfg)) }
pub fn aur(cfg: Config)     -> Box<dyn PackageManager> { Box::new(aur::Aur::new(cfg)) }
