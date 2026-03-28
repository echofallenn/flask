use flask::manager::flatpak::Flatpak;
use flask::manager::PackageManager;

#[tokio::test]
async fn flatpak_list_returns_ok() {
    if std::process::Command::new("which").arg("flatpak").output()
        .map(|o| o.status.success()).unwrap_or(false)
    {
        let f = Flatpak;
        assert!(f.list().await.is_ok());
    }
}
