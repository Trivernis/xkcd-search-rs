use crate::error::XKCDResult;
use crate::get_archive;
use std::cmp::Ordering;

/// Searches for a comic containing the words from the query
pub async fn search(query: &str) -> XKCDResult<Vec<(String, u32)>> {
    let archive = get_archive().await?;
    let words: Vec<String> = query
        .split(" ")
        .into_iter()
        .map(|s| s.to_lowercase())
        .collect();
    let mut entries: Vec<(usize, String, u32)> = archive
        .into_iter()
        .filter_map(|(key, id)| {
            let score = words
                .iter()
                .filter(|w| key.to_lowercase().contains(*w))
                .count();
            if score <= 0 {
                None
            } else {
                Some((score, key, id))
            }
        })
        .collect();
    entries.sort_by_key(|(s, _, _)| *s);
    entries.reverse();

    Ok(entries
        .into_iter()
        .map(|(_, title, id)| (title, id))
        .collect())
}

/// Searches for a comic with a fuzzy compare function that assigns a score
/// to each match.
/// The Threshold defines what still counts as a match
pub async fn search_fuzzy(query: &str, threshold: f32) -> XKCDResult<Vec<(String, u32)>> {
    let archive = get_archive().await?;
    let query = query.to_lowercase();

    let mut entries: Vec<(f32, String, u32)> = archive
        .into_iter()
        .map(|(key, id)| (trigram::similarity(&query, &key.to_lowercase()), key, id))
        .filter(|(score, _, _)| score >= &threshold)
        .collect();
    entries.sort_by(|(s1, _, _), (s2, _, _)| {
        if s1 > s2 {
            Ordering::Greater
        } else if s1 < s2 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    entries.reverse();

    Ok(entries
        .into_iter()
        .map(|(_, title, id)| (title, id))
        .collect())
}
