use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

use crate::config;
use crate::manager::{self, PackageManager};

#[derive(Parser)]
#[command(name = "flask", about = "your unnecessary multi-repo package manager", version)]
pub struct Cli {
    #[command(subcommand)]
    pub backend: BackendCmd,
}

#[derive(Subcommand)]
pub enum BackendCmd {
    Snap    { #[command(subcommand)] op: Op },
    Flatpak { #[command(subcommand)] op: Op },
    Pacman  { #[command(subcommand)] op: Op },
    Aur     { #[command(subcommand)] op: Op },
}

#[derive(Subcommand)]
pub enum Op {
    Install { packages: Vec<String> },
    Remove  { packages: Vec<String> },
    Search  { query: String },
    Update  { packages: Vec<String> },
    List,
    Info    { package: String },
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let cfg = config::load()?;

        let (mgr, op): (Box<dyn PackageManager>, Op) = match self.backend {
            BackendCmd::Snap    { op } => (manager::snap(cfg),    op),
            BackendCmd::Flatpak { op } => (manager::flatpak(cfg), op),
            BackendCmd::Pacman  { op } => (manager::pacman(cfg),  op),
            BackendCmd::Aur     { op } => (manager::aur(cfg),     op),
        };

        println!("{} {}", "flask →".cyan().bold(), mgr.name().bold());

        match op {
            Op::Install { packages } => mgr.install(&packages).await,
            Op::Remove  { packages } => mgr.remove(&packages).await,
            Op::Search  { query }    => mgr.search(&query).await,
            Op::Update  { packages } => mgr.update(&packages).await,
            Op::List                 => mgr.list().await,
            Op::Info    { package }  => mgr.info(&package).await,
        }
    }
}
