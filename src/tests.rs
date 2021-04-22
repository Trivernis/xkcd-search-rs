use crate::{get_archive, get_comic, get_latest_comic};

#[tokio::test]
async fn it_retrieves_the_archive() {
    let archive = get_archive().await.unwrap();
    assert!(archive.get("Password Strength").is_some());
}

#[tokio::test]
async fn it_retrieves_a_comic() {
    assert!(get_comic(1000).await.is_ok())
}

#[tokio::test]
async fn it_retrieves_the_latest_comic() {
    assert!(get_latest_comic().await.is_ok())
}
