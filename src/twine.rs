use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Hook {}

#[derive(Serialize, Deserialize)]
struct Link {
    link_text: String,
    passage_name: String,
    original: String,
}

#[derive(Serialize, Deserialize)]
struct Passage {
    id: String,
    name: String,
    tags: Vec<String>,
    text: String,
    clean_text: String,
    links: Vec<Link>,
    hooks: Vec<Hook>,
}

#[derive(Serialize, Deserialize)]
struct Game {
    uuid: String,
    name: String,
    creator: String,
    creator_version: String,
    schema_version: String,
    created_at_ms: u64,
    passages: Vec<Passage>,
}
