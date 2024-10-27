use reqwest::Client;
use serde_json::json;
use serde::Deserialize;

use crate::codename_game::anilist_api::graphql;

#[derive(Debug, Deserialize, Clone)]
struct Response {
    data: Data,
}

#[derive(Debug, Deserialize, Clone)]
struct Data {
    MediaListCollection: MediaListCollection,
}

#[derive(Debug, Deserialize, Clone)]
struct MediaListCollection {
    lists: Vec<List>,
}

#[derive(Debug, Deserialize, Clone)]
struct List {
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Clone)]
struct Entry {
    media: Media,
}

#[derive(Debug, Deserialize, Clone)]
struct Title {
    romaji: Option<String>,
    english: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Media {
    title: Title
}

fn format_name(name1: Option<String>, name2: Option<String>) -> Option<String> {
    match (name1, name2) {
        (Some(n1), Some(n2)) if n1 == n2 => Some(n1),
        (Some(n1), Some(n2)) => Some(format!("{} / {}", n1, n2)),
        (Some(n), None) | (None, Some(n)) => Some(n),
        (None, None) => None,
    }
}

pub async fn get_anime_names(username: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let client = Client::new();
    let json = json! (
        {
            "query": graphql::LIST_QUERY,
            "variables": {
                "userName": username,
            }
        }
    );
    let resp = client.post("https://graphql.anilist.co/")
                        .header("Content-Type", "application/json")
                        .header("Accept", "application/json")
                        .body(json.to_string())
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await;
    let mut result: Response = serde_json::from_str(&resp.unwrap()).expect("Error deseriazling list");
    for entry in result.data.MediaListCollection.lists[0].entries.clone() {
        match format_name(entry.media.title.english, entry.media.title.romaji) {
            Some(name) => names.push(name),
            None => (),
        }
    }
    names
}