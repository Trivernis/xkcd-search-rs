# XKCD Search

This crate provides functions to search for xkcd comics by title.

## Usage

```rust
use xkcd_search;

#[tokio::main]
async fn main() {
    let phone_entries = xkcd_search::search("Phone").await.unwrap();
    let mut comics = Vec::new();
    
    for (_title, id) in phone_entries {
        comics.push(xkcd_search::get_comic(id).await.unwrap())
    }
}
```

## License

Apache-2.0