use std::error::Error;
use std::fs::{read_to_string};
use std::path::Path;
use html_parser::{Dom};
use serde_json::{Value};
use super::schema;

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<schema::Game, Box<dyn Error>> {
    let html_data = read_to_string(path)?;
    // TODO: So ... given how we ended up using html_parser, it probably
    // makes more sense to go with something like roxmltree.
    let json = Dom::parse(&html_data)?.to_json()?;
    // Ok(json.chars().take(31).collect())
    let v: Value = serde_json::from_str(&json)?;
    let html_index = 0;
    let html: Value = v["children"][html_index].clone();
    // head = 0, body = 1
    let body_index = 1;
    let story_data_index = 1;
    let story_data = html["children"][body_index]["children"][story_data_index].clone();
    let g = schema::Game{
        name: story_data["attributes"]["name"].to_string(),
        // name: story_data.to_string(),
    };
    Ok(g)
}
