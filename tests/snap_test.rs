use flask::manager::snap::Snap;
use flask::manager::PackageManager;

#[tokio::test]
async fn snap_search_returns_ok() {
    // Only runs if snap is installed
    if std::process::Command::new("which").arg("snap").output()
        .map(|o| o.status.success()).unwrap_or(false)
    {
        let s = Snap;
        assert!(s.search("hello-world").await.is_ok());
    }
}
