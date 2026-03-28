use flask::manager::pacman::Pacman;
use flask::manager::PackageManager;

#[tokio::test]
async fn pacman_search_returns_ok() {
    if std::process::Command::new("which").arg("pacman").output()
        .map(|o| o.status.success()).unwrap_or(false)
    {
        let p = Pacman;
        assert!(p.search("neovim").await.is_ok());
    }
}
