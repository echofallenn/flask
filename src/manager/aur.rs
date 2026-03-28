use anyhow::Result;
use async_trait::async_trait;
use crate::config::Config;
use crate::manager::PackageManager;

pub struct Aur { helper: String }

impl Aur {
    pub fn new(cfg: Config) -> Self {
        Aur { helper: cfg.backends.aur.helper }
    }
}

#[async_trait]
impl PackageManager for Aur {
    fn name(&self) -> &'static str { "aur" }

    async fn install(&self, packages: &[String]) -> Result<()> {
        let mut args = vec!["-S", "--noconfirm"];
        args.extend(packages.iter().map(String::as_str));
        self.run_cmd(&self.helper, &args).await
    }

    async fn remove(&self, packages: &[String]) -> Result<()> {
        let mut args = vec!["-Rns", "--noconfirm"];
        args.extend(packages.iter().map(String::as_str));
        self.run_cmd(&self.helper, &args).await
    }

    async fn search(&self, query: &str) -> Result<()> {
        self.run_cmd(&self.helper, &["-Ss", query]).await
    }

    async fn update(&self, _packages: &[String]) -> Result<()> {
        self.run_cmd(&self.helper, &["-Syu", "--noconfirm"]).await
    }

    async fn list(&self) -> Result<()> {
        self.run_cmd(&self.helper, &["-Q"]).await
    }

    async fn info(&self, package: &str) -> Result<()> {
        self.run_cmd(&self.helper, &["-Si", package]).await
    }
}
