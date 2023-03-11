use super::schema;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<schema::Game, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let g = serde_json::from_reader(reader)?;
    Ok(g)
}

pub fn parse_bytes(data: &[u8]) -> serde_json::Result<schema::Game> {
    serde_json::from_slice(data)
}

pub fn parse_str(data: &str) -> serde_json::Result<schema::Game> {
    serde_json::from_str(data)
}
