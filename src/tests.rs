use crate::{get_archive, get_comic, get_latest_comic, search, search_fuzzy};

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

#[tokio::test]
async fn it_searches_for_comics() {
    let search_results = search("Phone").await.unwrap();
    assert!(!search_results.is_empty());
    assert!(search_results.len() < 1000);
}

#[tokio::test]
async fn it_fuzzy_searches_for_comics() {
    let search_results = search_fuzzy("Phone", 0.5).await.unwrap();
    assert!(!search_results.is_empty());
    assert!(search_results.len() < 1000);
}
