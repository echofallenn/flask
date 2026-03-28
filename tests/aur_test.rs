use flask::manager::aur::Aur;
use flask::manager::PackageManager;

#[tokio::test]
async fn aur_search_returns_ok() {
    let has_yay = std::process::Command::new("which").arg("yay").output()
        .map(|o| o.status.success()).unwrap_or(false);
    let has_paru = std::process::Command::new("which").arg("paru").output()
        .map(|o| o.status.success()).unwrap_or(false);

    if has_yay || has_paru {
        let a = Aur;
        assert!(a.search("neovim").await.is_ok());
    }
}
